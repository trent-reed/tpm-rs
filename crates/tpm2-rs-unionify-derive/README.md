# tpm2-rs-unionify-derive

I'm actually unsure of this, I think that the buffered size of the data should
be calculated as a part of the Marshalable trait. It's directly related to that.

I think `MAX_MARSHALED_SIZE`/`MIN_MARSHALED_SIZE` on Marshalable makes more
sense. I highly suspect we don't want this unionify thing.
