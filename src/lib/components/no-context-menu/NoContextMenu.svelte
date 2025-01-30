<script lang="ts">
    const oncontextmenu = (event: Event) => {
        const isEditableInput = (event.target as HTMLInputElement)?.nodeName === "INPUT" && (event.target as HTMLInputElement)?.type === "text";
        const isTextArea = (event.target as Element)?.nodeName === "TEXTAREA";
        const hasContentEditableParent = (event.target as Element)?.closest("[contenteditable]") || (event.target as Element)?.closest("[role='textbox']") ;

        if (!isEditableInput && !isTextArea && !hasContentEditableParent) {
            event.preventDefault();
        }
    };
</script>

<svelte:window {oncontextmenu} />
