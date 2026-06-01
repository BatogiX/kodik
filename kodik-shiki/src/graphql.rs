use std::collections::HashSet;

use crate::{AnimeKind, AnimeStatus, UserRate, deserialize_usize_from_string_or_number};
use kodik_utils::{Client, ClientExt as _};
use serde::{Deserialize, Serialize};
const LIMIT: usize = 50;

#[derive(Debug, Serialize)]
pub struct GraphQLRequest<V> {
    pub query: &'static str,
    pub variables: V,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchAnimesVars<'a> {
    pub franchise: &'a str,
    pub page: usize,
    pub limit: usize,
    pub exclude_ids: &'a str,
}

impl<'a> FetchAnimesVars<'a> {
    #[must_use]
    pub const fn new(franchise: &'a str, exclude_ids: &'a str) -> Self {
        Self {
            franchise,
            page: 1,
            limit: LIMIT,
            exclude_ids,
        }
    }
}

#[derive(Deserialize, Debug)]
struct FetchAnimesResponse {
    pub data: Related,
}

#[derive(Deserialize, Debug, Default)]
pub struct Related {
    pub animes: Vec<Anime>,
}

impl Related {
    /// # Errors
    /// Returns an error if the GraphQL request fails or the response cannot be deserialized.
    pub async fn fetch_by_franchise(
        client: &Client,
        franchise: &str,
        domain: &str,
        not_anime_ids: &[usize],
    ) -> crate::Result<Self> {
        const ANIMES_BY_FRANCHISE_QUERY: &str = r#"
    query($franchise: String!, $page: PositiveInt!, $limit: PositiveInt!, $excludeIds: String!) {
      animes(franchise: $franchise, page: $page, limit: $limit, excludeIds: $excludeIds, order: aired_on, status: "!anons") {
        id
        name
        status
        episodes
        episodesAired
        kind

        related {
          relationKind
          anime {
            id
          }
        }

        userRate {
          episodes
          id
          rewatches
          status
        }
      }
    }
    "#;

        let exclude_ids = not_anime_ids
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");

        let graphql_url = kodik_utils::api_url(domain, "/api/graphql");
        let mut json = GraphQLRequest {
            query: ANIMES_BY_FRANCHISE_QUERY,
            variables: FetchAnimesVars::new(franchise, &exclude_ids),
        };

        let mut related = Self::default();
        for page in 1.. {
            json.variables.page = page;
            let mut resp: FetchAnimesResponse = client.post_json_as_json(&graphql_url, &json).await?;
            let len = resp.data.animes.len();
            related.animes.append(&mut resp.data.animes);

            if len < LIMIT {
                break;
            }
        }

        Ok(related)
    }

    pub fn sort_by_chrono(&mut self) {
        self.animes.reverse();

        self.reorder_related(true, |k| {
            matches!(
                k,
                RelationKind::Prequel | RelationKind::ParentStory | RelationKind::FullStory | RelationKind::Orig
            )
        });

        self.reorder_related(false, |k| matches!(k, RelationKind::Sequel));
    }

    fn reorder_related(&mut self, pull_forward: bool, kind_pred: impl Fn(&RelationKind) -> bool) {
        for index in 0..self.animes.len() {
            let mut moved = HashSet::new();

            while let Some((pos, id)) = Self::find_related(&self.animes, index, &moved, &kind_pred, |p| {
                if pull_forward { p > index } else { p < index }
            }) {
                moved.insert(id);

                if pull_forward {
                    let item = self.animes.remove(pos);
                    self.animes.insert(index, item);
                } else {
                    let item = self.animes.remove(index);
                    self.animes.insert(pos, item);
                }
            }
        }
    }

    fn find_related(
        animes: &[Anime],
        current: usize,
        moved: &HashSet<usize>,
        kind_pred: impl Fn(&RelationKind) -> bool,
        pos_pred: impl Fn(usize) -> bool,
    ) -> Option<(usize, usize)> {
        animes[current]
            .related
            .iter()
            .filter(|r| kind_pred(&r.relation_kind))
            .find_map(|r| {
                let id = r.anime.as_ref()?.id;
                if moved.contains(&id) {
                    return None;
                }
                let pos = animes.iter().position(|a| a.id == id)?;
                pos_pred(pos).then_some((pos, id))
            })
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Anime {
    #[serde(deserialize_with = "deserialize_usize_from_string_or_number")]
    pub id: usize,
    pub name: String,
    pub status: AnimeStatus,
    pub kind: AnimeKind,
    pub episodes: usize,
    pub episodes_aired: usize,
    pub related: Vec<Relation>,
    pub user_rate: Option<UserRate>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Relation {
    pub relation_kind: RelationKind,
    pub anime: Option<RelationAnime>,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RelationKind {
    Adaptation,
    AlternativeSetting,
    AlternativeVersion,
    Character,
    FullStory,
    Other,
    ParentStory,
    Prequel,
    Sequel,
    SideStory,
    SpinOff,
    Summary,
    Fan,
    Orig,
}

#[derive(Deserialize, Debug)]
pub struct RelationAnime {
    #[serde(deserialize_with = "deserialize_usize_from_string_or_number")]
    pub id: usize,
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_related_test() {
        let client = Client::new();
        let franchise = "berserk";
        let domain = "shikimori.io";
        dbg!(
            Related::fetch_by_franchise(&client, franchise, domain, &[])
                .await
                .unwrap()
        );
    }
}
