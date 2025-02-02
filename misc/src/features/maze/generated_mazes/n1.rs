use crate::features::maze::*;

pub fn start_pos() -> UnpaddedPos {
    (0, 1)
}

/// ChatGPT!
pub fn matrix() -> UnpaddedMatrix {
    vec![
        vec![
            block(),
            start(),
            path(),
            path(),
            path(),
            block(),
            block(),
            block(),
            block(),
            block(),
        ],
        vec![
            block(),
            block(),
            block(),
            block(),
            path(),
            block(),
            path(),
            path(),
            path(),
            block(),
        ],
        vec![
            block(),
            path(),
            path(),
            block(),
            path(),
            block(),
            path(),
            block(),
            path(),
            block(),
        ],
        vec![
            block(),
            path(),
            block(),
            block(),
            path(),
            path(),
            path(),
            block(),
            path(),
            block(),
        ],
        vec![
            block(),
            path(),
            block(),
            block(),
            block(),
            block(),
            path(),
            block(),
            path(),
            block(),
        ],
        vec![
            block(),
            path(),
            path(),
            path(),
            path(),
            block(),
            path(),
            path(),
            path(),
            block(),
        ],
        vec![
            block(),
            block(),
            block(),
            block(),
            path(),
            block(),
            block(),
            block(),
            path(),
            block(),
        ],
        vec![
            block(),
            path(),
            path(),
            path(),
            path(),
            path(),
            path(),
            block(),
            path(),
            block(),
        ],
        vec![
            block(),
            path(),
            block(),
            block(),
            block(),
            block(),
            path(),
            block(),
            exit(),
            block(),
        ],
        vec![
            block(),
            block(),
            block(),
            block(),
            block(),
            block(),
            block(),
            block(),
            block(),
            block(),
        ],
    ]
}
