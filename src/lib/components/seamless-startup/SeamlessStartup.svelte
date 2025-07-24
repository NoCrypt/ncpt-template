<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";

  $effect(() => {
    try {
      const appWindow = getCurrentWindow();
      (async () => {
        await new Promise((resolve) => setTimeout(resolve, 80));
        appWindow.show();
      })();
    } catch (e) {
      console.error("Error hiding window:", e);
    }
  });
</script>
