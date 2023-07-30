use itertools::Itertools;

use crate::{query::Query, story_graph::StoryGraph, story_node::AliasCandidates};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct StoryId(usize);

// #[derive(Serialize, Deserialize)]
#[derive(Default)]
pub struct Raconteur {
    stories: Vec<StoryGraph>,
}

impl Raconteur {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, story_graph: StoryGraph) {
        self.stories.push(story_graph);
    }

    // Returns a pair of valid story beat with its list of valid aliased entities
    // inner vec is a list of permutations of indices. first index is for first alias, etc.
    pub fn query(&self, query: &Query) -> Vec<(StoryId, Vec<AliasCandidates>)> {
        // go through list of story beats, discarding those whose constraints aren't satisfied

        self.stories
            .iter()
            .enumerate()
            .filter_map(|(story_idx, story_graph)| {
                let story_node = story_graph.start();
                // TODO: go through children, look for at least one valid path to a childless node
                story_node
                    .try_matching_aliases(query)
                    .ok()
                    .map(|alias_candidates| (StoryId(story_idx), alias_candidates))
            })
            .collect_vec()
    }

    pub fn get(&self, story_id: StoryId) -> &StoryGraph {
        &self.stories[story_id.0]
    }
}