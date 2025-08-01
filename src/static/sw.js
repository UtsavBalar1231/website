// Service Worker for Terminal Portfolio
// PWA with offline support and stale-while-revalidate strategy

const CACHE_NAME = 'terminal-portfolio-v1';
const STATIC_CACHE_NAME = 'terminal-static-v1';
const RUNTIME_CACHE_NAME = 'terminal-runtime-v1';

// Assets to cache on install
const STATIC_ASSETS = [
  '/',
  '/about/',
  '/projects/', 
  '/resume/',
  '/contact/',
  '/tutorials/',
  '/books/',
  '/quotes/',
  '/css/main.css',
  '/js/bundle.js',
  '/manifest.json',
  '/icons/icon-192x192.png',
  '/icons/icon-512x512.png'
];

// Runtime cache patterns
const RUNTIME_CACHE_PATTERNS = [
  /^https:\/\/fonts\.googleapis\.com\//,
  /^https:\/\/fonts\.gstatic\.com\//,
  /\.(?:png|jpg|jpeg|svg|gif|webp)$/,
  /\.(?:js|css)$/
];

// Install event - cache static assets
self.addEventListener('install', event => {
  console.log('[SW] Installing service worker');
  
  event.waitUntil(
    caches.open(STATIC_CACHE_NAME)
      .then(cache => {
        console.log('[SW] Caching static assets');
        return cache.addAll(STATIC_ASSETS);
      })
      .then(() => {
        console.log('[SW] Static assets cached successfully');
        return self.skipWaiting();
      })
      .catch(error => {
        console.error('[SW] Failed to cache static assets:', error);
      })
  );
});

// Activate event - clean up old caches
self.addEventListener('activate', event => {
  console.log('[SW] Activating service worker');
  
  event.waitUntil(
    caches.keys()
      .then(cacheNames => {
        return Promise.all(
          cacheNames
            .filter(cacheName => {
              return cacheName.startsWith('terminal-') && 
                     cacheName !== STATIC_CACHE_NAME && 
                     cacheName !== RUNTIME_CACHE_NAME;
            })
            .map(cacheName => {
              console.log('[SW] Deleting old cache:', cacheName);
              return caches.delete(cacheName);
            })
        );
      })
      .then(() => {
        console.log('[SW] Claiming clients');
        return self.clients.claim();
      })
  );
});

// Fetch event - stale-while-revalidate strategy
self.addEventListener('fetch', event => {
  const { request } = event;
  const url = new URL(request.url);
  
  // Skip non-GET requests
  if (request.method !== 'GET') {
    return;
  }
  
  // Skip chrome-extension and other non-http(s) requests
  if (!url.protocol.startsWith('http')) {
    return;
  }
  
  // Handle navigation requests (HTML pages)
  if (request.mode === 'navigate') {
    event.respondWith(handleNavigationRequest(request));
    return;
  }
  
  // Handle static assets
  if (isStaticAsset(request)) {
    event.respondWith(handleStaticAsset(request));
    return;
  }
  
  // Handle runtime assets with stale-while-revalidate
  if (shouldCacheRuntime(request)) {
    event.respondWith(handleRuntimeAsset(request));
    return;
  }
  
  // Default: network first, no caching
  event.respondWith(fetch(request));
});

// Handle navigation requests (pages)
async function handleNavigationRequest(request) {
  try {
    // Try network first
    const networkResponse = await fetch(request);
    
    // Cache successful responses
    if (networkResponse.ok) {
      const cache = await caches.open(RUNTIME_CACHE_NAME);
      cache.put(request, networkResponse.clone());
    }
    
    return networkResponse;
  } catch (error) {
    console.log('[SW] Network failed for navigation, trying cache:', request.url);
    
    // Fallback to cache
    const cachedResponse = await caches.match(request);
    if (cachedResponse) {
      return cachedResponse;
    }
    
    // Fallback to offline page or root
    const fallbackResponse = await caches.match('/');
    if (fallbackResponse) {
      return fallbackResponse;
    }
    
    // Last resort - return a basic offline response
    return new Response(
      `<!DOCTYPE html>
      <html>
      <head>
        <title>Offline - Utsav Balar</title>
        <style>
          body { 
            font-family: monospace; 
            background: #0f0f0f; 
            color: #00ff41; 
            padding: 2rem; 
            text-align: center;
          }
          .terminal { 
            border: 1px solid #333; 
            padding: 1rem; 
            max-width: 600px; 
            margin: 2rem auto; 
          }
        </style>
      </head>
      <body>
        <div class="terminal">
          <h1>Connection Lost</h1>
          <p>$ curl -I utsavbalar.in</p>
          <p>curl: (6) Could not resolve host</p>
          <p><br>You're currently offline. Please check your connection and try again.</p>
          <p><a href="/" style="color: #00ff41;">← Return to Home</a></p>
        </div>
      </body>
      </html>`,
      {
        headers: { 'Content-Type': 'text/html' },
        status: 200
      }
    );
  }
}

// Handle static assets (cache first)
async function handleStaticAsset(request) {
  const cachedResponse = await caches.match(request);
  
  if (cachedResponse) {
    // Serve from cache and update in background
    fetchAndCache(request);
    return cachedResponse;
  }
  
  // Not in cache, fetch and cache
  return fetchAndCache(request);
}

// Handle runtime assets (stale-while-revalidate)
async function handleRuntimeAsset(request) {
  const cachedResponse = await caches.match(request);
  const fetchPromise = fetchAndCache(request);
  
  // Return cached version immediately if available
  if (cachedResponse) {
    return cachedResponse;
  }
  
  // Otherwise wait for network
  return fetchPromise;
}

// Fetch and cache helper
async function fetchAndCache(request) {
  try {
    const networkResponse = await fetch(request);
    
    // Only cache successful responses
    if (networkResponse.ok) {
      const cache = await caches.open(RUNTIME_CACHE_NAME);
      
      // Clone the response since it can only be consumed once
      cache.put(request, networkResponse.clone());
    }
    
    return networkResponse;
  } catch (error) {
    console.error('[SW] Fetch failed:', error);
    throw error;
  }
}

// Check if request is for a static asset
function isStaticAsset(request) {
  const url = new URL(request.url);
  return STATIC_ASSETS.some(asset => url.pathname === asset);
}

// Check if request should be cached at runtime
function shouldCacheRuntime(request) {
  return RUNTIME_CACHE_PATTERNS.some(pattern => pattern.test(request.url));
}

// Background sync for form submissions (if needed in future)
self.addEventListener('sync', event => {
  if (event.tag === 'contact-form') {
    event.waitUntil(syncContactForm());
  }
});

async function syncContactForm() {
  // Implementation for offline form submission sync
  console.log('[SW] Syncing contact form submissions');
}

// Push notifications (if needed in future)
self.addEventListener('push', event => {
  if (!event.data) return;
  
  const data = event.data.json();
  const options = {
    body: data.body,
    icon: '/icons/icon-192x192.png',
    badge: '/icons/badge-72x72.png',
    data: data.url,
    requireInteraction: true,
    actions: [
      {
        action: 'open',
        title: 'Open',
        icon: '/icons/action-open.png'
      },
      {
        action: 'close', 
        title: 'Close',
        icon: '/icons/action-close.png'
      }
    ]
  };
  
  event.waitUntil(
    self.registration.showNotification(data.title, options)
  );
});

// Handle notification clicks
self.addEventListener('notificationclick', event => {
  event.notification.close();
  
  if (event.action === 'open' || !event.action) {
    event.waitUntil(
      clients.openWindow(event.notification.data || '/')
    );
  }
});

// Handle service worker messages
self.addEventListener('message', event => {
  if (event.data && event.data.type === 'SKIP_WAITING') {
    self.skipWaiting();
  } else if (event.data && event.data.type === 'GET_VERSION') {
    event.ports[0].postMessage({ version: CACHE_NAME });
  }
});

console.log('[SW] Service worker loaded successfully');