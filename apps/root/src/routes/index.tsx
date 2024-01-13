import { component$ } from '@builder.io/qwik';
import { type DocumentHead } from '@builder.io/qwik-city';
import { Button } from '~/components';
import { generateSeoConfig } from '~/configs/seo';

export default component$(() => {
  return (
    <div class="flex h-full justify-center items-center flex-col gap-4 relative">
      <p class="text-3xl font-bold">
        <span class="icon-[noto--waving-hand]" /> Hello, I'm
      </p>
      <p class="text-5xl sm:text-7xl font-bold">
        <span class="bg-clip-text text-transparent bg-gradient-to-r from-primary to-tertiary">
          spicyzboss
        </span>
      </p>
      <div class="mt-4 gap-4 flex">
        <Button variant="secondary" href="https://cal.com/spicyz/meeting" target="_blank">
          Book a chat
        </Button>
        <Button
          variant="secondary"
          href="https://github.com/spicyzboss"
          target="_blank"
        >
          GitHub
        </Button>
      </div>
    </div>
  );
});

export const head: DocumentHead = generateSeoConfig();
