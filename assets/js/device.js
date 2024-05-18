function isMobileDevice() {
  return /Mobi|Android|iPhone|iPad|iPod/i.test(navigator.userAgent);
}

if (!isMobileDevice()) {
  document.getElementById('unsupported-message').style.display = 'block';
  document.getElementById('game-content').style.display = 'none';
} else {
  // Initialize your game here
  console.log('Mobile device detected. Game is loading...');
}


isMobileDevice()