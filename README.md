# pinchrs

pinchrs is a simple image proxy server written in Rust.

- Built with [axum](https://github.com/tokio-rs/axum) and [image-rs](https://github.com/image-rs/image)
- Less performant than the competition (due to not using libvips and not using streaming)

## Usage

The URL format for processing images is quite similar to other image proxy servers:

```
/<signature_base64>/<operation>:<param>:<param>/.../<image_url_base64>
```

Example usage with URL signing:

```
# Start server with "secret" as the signature key
docker run -p 3000:3000 -e KEY=secret ghcr.io/kankje/pinchrs:latest

# Base64-encode the image URL (URL-safe)
echo 'https://images.unsplash.com/photo-1593288942460-e321b92a6cde?w=1920' \
  | base64 \
  | tr '+/' '-_' \
  | tr -d '='
# -> aHR0cHM6Ly9pbWFnZXMudW5zcGxhc2guY29tL3Bob3RvLTE1OTMyODg5NDI0NjAtZTMyMWI5MmE2Y2RlP3c9MTkyMAo

# Generate a signature for our URL and base64-encode it (URL-safe)
echo -n "resize:200:200/format:avif/aHR0cHM6Ly9pbWFnZXMudW5zcGxhc2guY29tL3Bob3RvLTE1OTMyODg5NDI0NjAtZTMyMWI5MmE2Y2RlP3c9MTkyMAo" \
  | openssl dgst -sha256 -hmac "secret" -binary \
  | base64 \
  | tr '+/' '-_' \
  | tr -d '='
# -> hvEqh1D117KYz1QYRQ6qFul6GzjEnwoDGSXGt6szZNg

# The final URL is:
# http://localhost:3000/hvEqh1D117KYz1QYRQ6qFul6GzjEnwoDGSXGt6szZNg/resize:200:200/format:avif/aHR0cHM6Ly9pbWFnZXMudW5zcGxhc2guY29tL3Bob3RvLTE1OTMyODg5NDI0NjAtZTMyMWI5MmE2Y2RlP3c9MTkyMAo
```

If `KEY` is not set, signature checking is disabled and the signature can be anything.

## Deployment

Currently, I wouldn't recommend using this in production. Instead, use one of these:

- [imgproxy](https://github.com/imgproxy/imgproxy)
- [imagor](https://github.com/cshum/imagor)
- [thumbor](https://github.com/thumbor/thumbor)

If you do decide to use pinchrs anyway:

- Deploy the Docker image to your container orchestration solution (e.g. Kubernetes, AWS ECS)
- **Important:** Set the `KEY` environment variable to enable signature checking and sign your URLs using the key.
  Otherwise, your
  server will be vulnerable to denial-of-service attacks!
- Set up a CDN (e.g. Amazon CloudFront) in front of pinchrs to cache the processed images
- Set up monitoring and alarms for the service

### Environment variables

| Name       | Description                    | Default                        |
|------------|--------------------------------|--------------------------------|
| `HOST`     | Host to listen on              | `0.0.0.0`                      |
| `PORT`     | Port to listen on              | `3000`                         |
| `KEY`      | HMAC-SHA256 key for signatures | _(empty)_                      |
| `RUST_LOG` | Logging level                  | `pinchrs=info,tower_http=warn` |

## Supported protocols for input images

| Protocol        | Example                                                               |
|-----------------|-----------------------------------------------------------------------|
| `http`, `https` | `https://images.unsplash.com/photo-1593288942460-e321b92a6cde?w=1920` |

## Supported formats

| Format   | Decoding | Encoding            |
|----------|----------|---------------------|
| AVIF     | No       | Yes (lossy only)    |
| BMP      | Yes      | Yes                 |
| DDS      | Yes      | ---                 |
| Farbfeld | Yes      | Yes                 |
| GIF      | Yes      | Yes                 |
| HDR      | Yes      | Yes                 |
| ICO      | Yes      | Yes                 |
| JPEG     | Yes      | Yes                 |
| EXR      | Yes      | Yes                 |
| PNG      | Yes      | Yes                 |
| PNM      | Yes      | Yes                 |
| QOI      | Yes      | Yes                 |
| TGA      | Yes      | Yes                 |
| TIFF     | Yes      | Yes                 |
| WebP     | Yes      | Yes (lossless only) |

## Supported operations

| Operation                 | Example          | Description                                                                |
|---------------------------|------------------|----------------------------------------------------------------------------|
| `format:<extension>`      | `format:avif`    | Set output file format                                                     |
| `quality:<quality>`       | `quality:80`     | Encoding quality for AVIF (1-100, default 80) and JPEG (1-100, default 80) |
| `speed:<speed>`           | `speed:8`        | Encoding speed for AVIF (1-10, default 8) and GIF (1-30, default 10)       |
| `resize:<width>:<height>` | `resize:200:200` | Resizes image so it fits within the specified bounds                       |
| `rotate:<degrees>`        | `rotate:90`      | Rotates image, degrees must be divisible by 90                             |
