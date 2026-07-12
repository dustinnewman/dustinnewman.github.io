// Serves and copies files co-located with posts (images, videos) so that
// relative references like `./figure.png` keep resolving to
// /posts/<slug>/figure.png, matching Hugo's page-bundle behavior.
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const MIME = {
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.jpeg': 'image/jpeg',
  '.gif': 'image/gif',
  '.svg': 'image/svg+xml',
  '.webp': 'image/webp',
  '.avif': 'image/avif',
  '.mp4': 'video/mp4',
  '.webm': 'video/webm',
  '.pdf': 'application/pdf',
  '.txt': 'text/plain; charset=utf-8',
};
const SOURCE_FILES = /\.(md|mdx)$/;

export default function postAssets() {
  let contentDir;
  return {
    name: 'post-assets',
    hooks: {
      'astro:config:done': ({ config }) => {
        contentDir = fileURLToPath(new URL('./src/content/posts', config.root));
      },
      'astro:server:setup': ({ server }) => {
        server.middlewares.use((req, res, next) => {
          const url = decodeURIComponent((req.url ?? '').split('?')[0]);
          if (!url.startsWith('/posts/')) return next();
          const file = path.normalize(path.join(contentDir, url.slice('/posts/'.length)));
          if (!file.startsWith(contentDir + path.sep) || SOURCE_FILES.test(file)) return next();
          fs.stat(file, (err, stat) => {
            if (err || !stat.isFile()) return next();
            res.setHeader('Content-Type', MIME[path.extname(file).toLowerCase()] ?? 'application/octet-stream');
            fs.createReadStream(file).pipe(res);
          });
        });
      },
      'astro:build:done': ({ dir }) => {
        fs.cpSync(contentDir, fileURLToPath(new URL('./posts/', dir)), {
          recursive: true,
          filter: (src) => !SOURCE_FILES.test(src),
        });
      },
    },
  };
}
