<script lang="ts">
    import { getCurrentWindow } from '@tauri-apps/api/window';

    $effect(()=>{
        const appWindow = getCurrentWindow();
        (async ()=> {
            await new Promise(resolve => setTimeout(resolve, 80));
            appWindow.show();
        })()
    })

</script>