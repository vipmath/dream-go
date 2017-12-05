// Copyright 2017 Karl Sundequist Blomdahl <karl.sundequist.blomdahl@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::env;

pub trait Param {
    /// The number of probes into the monte carlo tree to perform at each step. This
    /// integer must be dividable by both `thread_count` and `batch_size` and larger
    /// than zero.
    fn iteration_limit() -> usize;  // 512

    /// The number of threads to run in parallel probing into the search tree.
    fn thread_count() -> usize;  // 16

    /// The number of asynchronous neural network evaluations to batch together.
    fn batch_size() -> usize;  // 8

    /// How much dirichlet noise to add to the policy at the root of the
    /// monte carlo tree search.
    fn dirichlet_noise() -> f32;  // 0.25

    /// The exploration rate constant in the UCT formula, a higher value
    /// indicate a higher level of exploration.
    fn exploration_rate() -> f32;  // sqrt(2)

    /// The absolute bias of the AMAF value compared to the local value,
    /// this can be heuristically determined based on what the difference
    /// between the final value and the AMAF value is.
    fn rave_bias() -> f32;  // 0.1

    /// Whether to use experimental features.
    fn experimental() -> bool;
}

#[derive(Clone)]
pub struct Standard;

impl Param for Standard {
    fn iteration_limit() -> usize {
        lazy_static! {
            static ref LIMIT: usize = {
                let limit = match env::var("NUM_ITER") {
                    Ok(val) => val.parse::<usize>()
                                  .expect(&format!("NUM_ITER: expected number, received {}", val)),
                    Err(_) => 512
                };

                assert!(limit > 0,
                    "The number of iterations ({}) must be larger than zero",
                    limit
                );

                limit
            };
        }

        *LIMIT
    }

    fn thread_count() -> usize {
        lazy_static! {
            static ref COUNT: usize = {
                let count = match env::var("NUM_THREADS") {
                    Ok(val) => val.parse::<usize>()
                                  .expect(&format!("NUM_THREADS: expected number, received {}", val)),
                    Err(_) => 16
                };

                assert!(count > 0,
                    "The number of threads ({}) must be larger than zero",
                    count
                );
                assert_eq!(Standard::iteration_limit() % count, 0,
                    "The number of threads ({}) must be an integer divider of the number of iterations ({})",
                    count,
                    Standard::iteration_limit()
                );

                count
            };
        }

        *COUNT
    }

    fn batch_size() -> usize {
        lazy_static! {
            static ref SIZE: usize = {
                let size = match env::var("BATCH_SIZE") {
                    Ok(val) => val.parse::<usize>()
                                  .expect(&format!("BATCH_SIZE: expected number, received {}", val)),
                    Err(_) => 8
                };

                assert!(size > 0,
                    "The batch size ({}) must be larger than zero",
                    size
                );
                assert_eq!(Standard::iteration_limit() % size, 0,
                    "The batch size ({}) must be an integer divider of the number of iterations ({})",
                    size,
                    Standard::iteration_limit()
                );
                assert!(size <= Standard::thread_count(),
                    "The batch size ({}) must be smaller than or equal to the number of threads ({})",
                    size,
                    Standard::thread_count()
                );

                size
            };
        }

        *SIZE
    }

    #[inline] fn dirichlet_noise() -> f32 { 0.25 }
    #[inline] fn exploration_rate() -> f32 { 1.41421356237 }
    #[inline] fn rave_bias() -> f32 { 0.1 }
    #[inline] fn experimental() -> bool { false }
}

#[derive(Clone)]
#[allow(dead_code)]
pub struct Experimental;

impl Param for Experimental {
    #[inline] fn iteration_limit() -> usize { Standard::iteration_limit() }
    #[inline] fn thread_count() -> usize { Standard::thread_count() }
    #[inline] fn batch_size() -> usize { Standard::batch_size() }
    #[inline] fn dirichlet_noise() -> f32 { Standard::dirichlet_noise() }
    #[inline] fn exploration_rate() -> f32 { Standard::exploration_rate() }
    #[inline] fn rave_bias() -> f32 { Standard::rave_bias() }
    #[inline] fn experimental() -> bool { true }
}
