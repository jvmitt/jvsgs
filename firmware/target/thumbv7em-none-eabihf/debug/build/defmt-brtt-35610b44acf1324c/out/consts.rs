/// BBQ & RTT buffer size (default: 1024. Consumed twice, once for RTT and once for BBQueue).
            ///
            /// Can be customized by setting the `DEFMT_BRTT_BUFFER_SIZE` environment variable.
            /// Use a power of 2 for best performance.
            pub const BUF_SIZE: usize = 1024;