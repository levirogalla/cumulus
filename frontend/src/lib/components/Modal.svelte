<script lang="ts">
    import type { Snippet } from "svelte";
	import ButtonX from "./ui/ButtonX.svelte";
	import Darken from "./ui/Darken.svelte";

    let {
        children,
        show = $bindable(),
        title,
        closeOnBackdrop = true,
        closeOnEscape = true,
        class: className,
    }: {
        children?: Snippet;
        show: boolean;
        title?: string;
        closeOnBackdrop?: boolean;
        closeOnEscape?: boolean;
        class?: string;
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

    let classes = $derived(`w-full max-w-2xl rounded p-3 bg-white ${className}`)
</script>

{#if show}
<!-- @ts-ignore: TS2322 -->
    <Darken
        class="fixed inset-0 z-50 grid place-items-center bg-black/50 p-4"
        onclick={onBackdropClick}
        onkeydown={onKeydown}
        role="button"
        show
    >
        <div
            class={classes}
            role="dialog"
            aria-modal="true"
            aria-label={"Modal"}
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
                <ButtonX onclick={close}/>

            </header>

            <section>
                {@render children?.()}
            </section>
        </div>
    </Darken>
{/if}