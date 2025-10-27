const { getCurrentWindow } = window.__TAURI__.window;
const { invoke } = window.__TAURI__.core;

// Initialize window controls
window.addEventListener("DOMContentLoaded", () => {
  const appWindow = getCurrentWindow();

  // Minimize button
  const minimizeBtn = document.getElementById("minimize-btn");
  if (minimizeBtn) {
    minimizeBtn.addEventListener("click", async () => {
      await appWindow.minimize();
    });
  }

  // Close button
  const closeBtn = document.getElementById("close-btn");
  if (closeBtn) {
    closeBtn.addEventListener("click", async () => {
      await appWindow.close();
    });
  }

  // Auto-hide titlebar after 3 seconds of inactivity
  let hideTimeout;
  const titlebar = document.querySelector(".titlebar");

  const showTitlebar = () => {
    if (titlebar) {
      titlebar.style.opacity = "1";
      clearTimeout(hideTimeout);
      hideTimeout = setTimeout(() => {
        titlebar.style.opacity = "0";
      }, 3000);
    }
  };

  // Show titlebar on mouse movement
  document.addEventListener("mousemove", showTitlebar);

  // Keep titlebar visible when hovering over it
  if (titlebar) {
    titlebar.addEventListener("mouseenter", () => {
      clearTimeout(hideTimeout);
      titlebar.style.opacity = "1";
    });

    titlebar.addEventListener("mouseleave", () => {
      hideTimeout = setTimeout(() => {
        titlebar.style.opacity = "0";
      }, 3000);
    });
  }

  // Save window position and size when moved or resized
  let saveTimeout;
  const saveWindowState = async () => {
    clearTimeout(saveTimeout);
    saveTimeout = setTimeout(async () => {
      try {
        await invoke("save_window_position");
      } catch (error) {
        console.error("Failed to save window state:", error);
      }
    }, 500); // Debounce to avoid excessive saves
  };

  // Listen for window resize and move events
  appWindow.listen("tauri://resize", saveWindowState);
  appWindow.listen("tauri://move", saveWindowState);
});
