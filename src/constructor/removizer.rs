use super::input::UserRemoves;
use crate::parser::Stats;

impl Stats {
    pub fn reset(&mut self, to_remove: UserRemoves) {
        if to_remove.stashed {
            self.stashed = 0
        }
        if to_remove.unpulled {
            self.unpulled = 0
        }
        if to_remove.unpushed {
            self.unpushed = 0
        }
        if to_remove.renamed {
            self.renamed = 0
        }
        if to_remove.staged {
            self.staged = 0
        }
        if to_remove.added {
            self.added = 0
        }
        if to_remove.staged_deleted {
            self.staged_deleted = 0
        }
        if to_remove.modified {
            self.modified = 0
        }
        if to_remove.unstaged {
            self.unstaged = 0
        }
        if to_remove.deleted {
            self.deleted = 0
        }
    }
}
