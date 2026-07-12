import rss from '@astrojs/rss';
import { getCollection } from 'astro:content';
import MarkdownIt from 'markdown-it';

const parser = new MarkdownIt({ html: true });

export async function GET(context) {
  const posts = (await getCollection('posts', ({ data }) => !data.draft))
    .sort((a, b) => b.data.date.valueOf() - a.data.date.valueOf());
  return rss({
    title: 'Dustin Newman',
    description: 'Recent content on Dustin Newman',
    site: context.site,
    items: posts.map((post) => ({
      title: post.data.title,
      link: `/posts/${post.id}/`,
      pubDate: post.data.date,
      description: parser.render(post.body ?? ''),
    })),
  });
}
