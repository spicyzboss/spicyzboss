import { component$ } from '@builder.io/qwik';
import { DocumentHead } from '@builder.io/qwik-city';

export default component$(() => {
  return (
    <div class="container">
      <p class="title">Nothing here</p>
    </div>
  );
});

export const head: DocumentHead = {
  title: 'spicyz.io',
  meta: [
    {
      name: 'description',
      content: 'spicyzboss, a software engineer student who loves to think.',
    },
    {
      property: 'og:title',
      content: 'spicyz.io',
    },
    {
      property: 'og:description',
      content: 'spicyzboss, a software engineer student who loves to think.',
    },
  ],
};
