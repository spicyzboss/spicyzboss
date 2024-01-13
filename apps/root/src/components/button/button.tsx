import { Slot, component$ } from '@builder.io/qwik';
import { Link } from '@builder.io/qwik-city';

export const ButtonVariants = ['primary', 'secondary', 'tertiary'] as const;
export type ButtonVariant = (typeof ButtonVariants)[number];

interface Props {
  href: string;
  target?: HTMLAnchorElement['target'];
  variant?: ButtonVariant;
}

export const Button = component$((props: Props) => {
  const { href, target, variant = 'primary' } = props;

  const ContainerVariants = {
    primary: 'bg-primary',
    secondary: 'ring-1 ring-outline',
    tertiary: 'bg-tertiary',
  } satisfies { [K in ButtonVariant]: string };

  const LabelVariants = {
    primary: 'text-primary-on',
    secondary: 'text-primary',
    tertiary: 'text-tertiary-on',
  } satisfies { [K in ButtonVariant]: string };

  const StateLayerVariants = {
    primary:
      'group-hover:bg-surface-variant/[.08] group-focus:bg-surface-variant/[.1] group-active:bg-surface-variant/[.1]',
    secondary:
      'group-hover:bg-primary/[.08] group-active:bg-primary/[.1] group-focus:bg-primary/[.1]',
    tertiary: 'bg-tertiary-on/[.08]',
  } satisfies { [K in ButtonVariant]: string };

  return (
    <Link
      href={href}
      class="group px-4 py-2 rounded relative flex gap-2 justify-center items-center"
      target={target}
    >
      <div
        class={[
          'rounded-[inherit] absolute inset-0',
          ContainerVariants[variant],
        ]}
      />
      <div
        class={[
          'rounded-[inherit] absolute inset-0',
          StateLayerVariants[variant],
        ]}
      />
      <Slot name="leftIcon" />
      <span class={['font-bold z-0', LabelVariants[variant]]}>
        <Slot />
      </span>
      <Slot name="rightIcon" />
    </Link>
  );
});
