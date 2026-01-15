<script lang="ts">
    import type { Snippet } from "svelte";
    import { self } from 'svelte/legacy';

    let {
        children,
        show = $bindable(),
        title,
        closeOnBackdrop = true,
        closeOnEscape = true,
        ariaLabel
    }: {
        children?: Snippet;
        show: boolean;
        title?: string;
        closeOnBackdrop?: boolean;
        closeOnEscape?: boolean;
        ariaLabel?: string;
    } = $props();

    const close = () => {
        show = false;
    };

    const onBackdropClick = (e: MouseEvent) => {
        if (closeOnBackdrop) close();
    };

    const onKeydown = (event: KeyboardEvent) => {
        if (closeOnEscape && event.key === "Escape") {
            close();
        }
    };
</script>

{#if show}
    <div
        class="fixed inset-0 z-50 grid place-items-center bg-black/50 p-4"
        onclick={onBackdropClick}
        onkeydown={onKeydown}
        role="button"
        tabindex="0"
    >
        <div
            class="w-full max-w-2xl rounded bg-white p-3 "
            role="dialog"
            aria-modal="true"
            aria-label={ariaLabel ?? title ?? "Modal"}
            tabindex="0"
            onclick={(e) => e.stopPropagation()}
            onkeydown={()=>{}}
        >
            <header class="mb-4 flex items-center justify-between gap-3">
                <div></div>
                {#if title}
                  <h2 class="text-xl font-semibold">{title}</h2>
                {:else}
                  <div></div>
                {/if}
                <button
                    type="button"
                    class="rounded-md p-1 text-2xl leading-none text-slate-500 "
                    aria-label="Close"
                    onclick={close}
                >
                    ×
                </button>
            </header>

            <section>
                {@render children?.()}
            </section>
        </div>
    </div>
{/if}