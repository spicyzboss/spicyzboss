import { component$ } from '@builder.io/qwik';
import { DocumentHead } from '@builder.io/qwik-city';
import { Button } from '~/components';
import { generateSeoConfig } from '~/configs/seo';

export default component$(() => {
  return (
    <div class="flex h-full justify-center items-center flex-col gap-4">
      <p class="text-4xl sm:text-7xl font-bold flex gap-4">
        <span class="icon-[noto--construction]" />
        <span class="bg-clip-text text-transparent bg-gradient-to-r from-primary to-tertiary">
          DEAD END
        </span>
        <span class="icon-[noto--construction]" />
      </p>
      <p>You have now at the end of the network. Please head back to home.</p>
      <Button href="/">Back to Home</Button>
    </div>
  );
});

export const head: DocumentHead = generateSeoConfig({ title: 'Not Found' });
