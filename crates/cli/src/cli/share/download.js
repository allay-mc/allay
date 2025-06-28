const downloadDebugButton = document.getElementById('download-debug-button');
const downloadReleaseButton = document.getElementById('download-release-button');

if (downloadReleaseButton !== null) {
  downloadReleaseButton.addEventListener('click', function() {
    window.open('release', '_blank');
  });
}

if (downloadDebugButton !== null) {
  downloadDebugButton.addEventListener('click', function () {
    window.open('debug', '_blank');
  });
}
