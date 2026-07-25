// Controller Remapper - Frontend Application
const { invoke } = window.__TAURI__.core;
const { open } = window.__TAURI__.dialog;

// Application State
const state = {
    controllers: [],
    selectedController: null,
    profiles: [],
    selectedProfile: null,
    mappings: [],
    steamInitialized: false,
    currentSection: 'controllers',
    pendingMappingButton: null
};

// DOM Elements
const elements = {
    // Navigation
    navItems: document.querySelectorAll('.nav-item'),
    sections: document.querySelectorAll('.section'),
    
    // Controllers
    scanControllers: document.getElementById('scanControllers'),
    controllersList: document.getElementById('controllersList'),
    mappingSection: document.getElementById('mappingSection'),
    clearMappings: document.getElementById('clearMappings'),
    saveMapping: document.getElementById('saveMapping'),
    mappingsList: document.getElementById('mappingsList'),
    
    // Profiles
    createProfile: document.getElementById('createProfile'),
    profilesList: document.getElementById('profilesList'),
    searchProfiles: document.getElementById('searchProfiles'),
    filterGame: document.getElementById('filterGame'),
    
    // Workshop
    searchWorkshop: document.getElementById('searchWorkshop'),
    searchWorkshopBtn: document.getElementById('searchWorkshopBtn'),
    workshopContent: document.getElementById('workshopContent'),
    workshopTabs: document.querySelectorAll('.tab-btn'),
    
    // Cloud
    syncCloud: document.getElementById('syncCloud'),
    cloudStatus: document.getElementById('cloudStatus'),
    cloudQuota: document.getElementById('cloudQuota'),
    cloudProfilesList: document.getElementById('cloudProfilesList'),
    
    // Steam
    steamStatus: document.getElementById('steamStatus'),
    
    // Modal
    modal: document.getElementById('modal'),
    modalTitle: document.getElementById('modalTitle'),
    modalBody: document.getElementById('modalBody'),
    modalClose: document.getElementById('modalClose'),
    
    // Key Capture
    keyCaptureModal: document.getElementById('keyCaptureModal'),
    capturedKey: document.getElementById('capturedKey'),
    cancelKeyCapture: document.getElementById('cancelKeyCapture')
};

// Initialize Application
async function init() {
    setupEventListeners();
    await checkSteamStatus();
    await loadProfiles();
    renderProfiles();
}

// Event Listeners
function setupEventListeners() {
    // Navigation
    elements.navItems.forEach(item => {
        item.addEventListener('click', () => switchSection(item.dataset.section));
    });
    
    // Controllers
    elements.scanControllers.addEventListener('click', scanControllers);
    elements.clearMappings.addEventListener('click', clearMappings);
    elements.saveMapping.addEventListener('click', saveMappingToProfile);
    
    // Controller visual buttons
    document.querySelectorAll('.controller-stick, .dpad-button, .face-button, .shoulder-button, .center-button').forEach(btn => {
        btn.addEventListener('click', () => handleControllerButtonClick(btn.dataset.button));
    });
    
    // Profiles
    elements.createProfile.addEventListener('click', showCreateProfileModal);
    elements.searchProfiles.addEventListener('input', debounce(handleSearchProfiles, 300));
    
    // Workshop
    elements.searchWorkshopBtn.addEventListener('click', searchWorkshop);
    elements.workshopTabs.forEach(tab => {
        tab.addEventListener('click', () => switchWorkshopTab(tab.dataset.tab));
    });
    
    // Cloud
    elements.syncCloud.addEventListener('click', syncCloud);
    
    // Modal
    elements.modalClose.addEventListener('click', closeModal);
    elements.modal.addEventListener('click', (e) => {
        if (e.target === elements.modal) closeModal();
    });
    
    // Key Capture
    elements.cancelKeyCapture.addEventListener('click', closeKeyCaptureModal);
    document.addEventListener('keydown', handleKeyCapture);
}

// Navigation
function switchSection(section) {
    state.currentSection = section;
    
    elements.navItems.forEach(item => {
        item.classList.toggle('active', item.dataset.section === section);
    });
    
    elements.sections.forEach(sec => {
        sec.classList.toggle('active', sec.id === `${section}-section`);
    });
    
    if (section === 'cloud') {
        loadCloudStatus();
    }
}

// Steam
async function checkSteamStatus() {
    try {
        const initialized = await invoke('is_steam_initialized');
        state.steamInitialized = initialized;
        
        const statusIndicator = elements.steamStatus.querySelector('.status-indicator');
        statusIndicator.classList.toggle('online', initialized);
        statusIndicator.classList.toggle('offline', !initialized);
        elements.steamStatus.innerHTML = `
            <span class="status-indicator ${initialized ? 'online' : 'offline'}"></span>
            Steam: ${initialized ? 'Connected' : 'Disconnected'}
        `;
        
        if (!initialized) {
            await invoke('initialize_steam');
            await checkSteamStatus();
        }
    } catch (error) {
        console.error('Failed to check Steam status:', error);
    }
}

// Controllers
async function scanControllers() {
    try {
        elements.controllersList.innerHTML = '<div class="empty-state"><p>Scanning...</p></div>';
        
        const controllers = await invoke('scan_controllers');
        state.controllers = controllers;
        
        renderControllers();
    } catch (error) {
        console.error('Failed to scan controllers:', error);
        elements.controllersList.innerHTML = `
            <div class="empty-state">
                <p class="text-error">Failed to scan controllers</p>
                <p class="text-muted">${error}</p>
            </div>
        `;
    }
}

function renderControllers() {
    if (state.controllers.length === 0) {
        elements.controllersList.innerHTML = `
            <div class="empty-state">
                <svg width="48" height="48" viewBox="0 0 48 48" fill="none" stroke="currentColor" stroke-width="2">
                    <rect x="4" y="12" width="40" height="24" rx="6"/>
                    <circle cx="12" cy="24" r="3"/>
                    <circle cx="36" cy="24" r="3"/>
                    <circle cx="24" cy="18" r="3"/>
                    <circle cx="24" cy="30" r="3"/>
                </svg>
                <p>No controllers detected</p>
                <p class="text-muted">Click "Scan for Controllers" to detect connected devices</p>
            </div>
        `;
        return;
    }
    
    elements.controllersList.innerHTML = state.controllers.map(controller => `
        <div class="controller-card" data-id="${controller.id}">
            <div class="controller-info">
                <h3>${controller.name}</h3>
                <p>${controller.is_bluetooth ? 'Bluetooth' : 'Wired'} • ${controller.vendor_id.toString(16)}:${controller.product_id.toString(16)}</p>
            </div>
            <div class="controller-status">
                <span class="status-indicator ${controller.connected ? 'online' : 'offline'}"></span>
                <button class="btn btn-primary" onclick="selectController('${controller.id}')">
                    Select
                </button>
            </div>
        </div>
    `).join('');
}

async function selectController(controllerId) {
    try {
        state.selectedController = state.controllers.find(c => c.id === controllerId);
        elements.mappingSection.classList.remove('hidden');
        
        // Load existing mappings if a profile is selected
        if (state.selectedProfile) {
            await loadProfileMappings(state.selectedProfile.id);
        }
        
        // Highlight selected controller card
        document.querySelectorAll('.controller-card').forEach(card => {
            card.classList.toggle('selected', card.dataset.id === controllerId);
        });
    } catch (error) {
        console.error('Failed to select controller:', error);
    }
}

function handleControllerButtonClick(buttonName) {
    if (!state.selectedController) {
        alert('Please select a controller first');
        return;
    }
    
    state.pendingMappingButton = buttonName;
    showKeyCaptureModal();
}

// Key Capture
function showKeyCaptureModal() {
    elements.keyCaptureModal.classList.remove('hidden');
    elements.capturedKey.textContent = '...';
}

function closeKeyCaptureModal() {
    elements.keyCaptureModal.classList.add('hidden');
    state.pendingMappingButton = null;
}

function handleKeyCapture(event) {
    if (!elements.keyCaptureModal.classList.contains('hidden')) {
        event.preventDefault();
        
        const keyName = event.key.toUpperCase();
        elements.capturedKey.textContent = keyName;
        
        // Map the button to the key
        if (state.pendingMappingButton) {
            addMapping(state.pendingMappingButton, keyName);
            closeKeyCaptureModal();
        }
    }
}

// Mappings
async function addMapping(button, key) {
    try {
        await invoke('map_button', { button, key });
        state.mappings.push({ button, key });
        renderMappings();
    } catch (error) {
        console.error('Failed to add mapping:', error);
    }
}

function renderMappings() {
    elements.mappingsList.innerHTML = state.mappings.map((mapping, index) => `
        <div class="mapping-item">
            <div class="mapping-button">
                <span>${formatButtonName(mapping.button)}</span>
                <span>→</span>
                <span class="mapping-key">${mapping.key}</span>
            </div>
            <button class="mapping-remove" onclick="removeMapping(${index})">
                <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M4 4l8 8M4 12L12 4"/>
                </svg>
            </button>
        </div>
    `).join('');
}

function formatButtonName(button) {
    const names = {
        'a': 'A',
        'b': 'B',
        'x': 'X',
        'y': 'Y',
        'leftstick': 'Left Stick',
        'rightstick': 'Right Stick',
        'dpadup': 'D-Pad Up',
        'dpaddown': 'D-Pad Down',
        'dpadleft': 'D-Pad Left',
        'dpadright': 'D-Pad Right',
        'leftshoulder': 'Left Shoulder',
        'rightshoulder': 'Right Shoulder',
        'back': 'Back',
        'start': 'Start',
        'guide': 'Guide'
    };
    return names[button] || button;
}

function removeMapping(index) {
    state.mappings.splice(index, 1);
    renderMappings();
}

async function clearMappings() {
    try {
        await invoke('clear_mappings');
        state.mappings = [];
        renderMappings();
    } catch (error) {
        console.error('Failed to clear mappings:', error);
    }
}

async function saveMappingToProfile() {
    if (!state.selectedProfile) {
        alert('Please select or create a profile first');
        return;
    }
    
    try {
        // Update profile with current mappings
        const profile = await invoke('get_profile', { profileId: state.selectedProfile.id });
        profile.button_mappings = state.mappings.map(m => ({
            controller_button: m.button,
            keyboard_key: m.key,
            enabled: true
        }));
        
        await invoke('save_profile', { profile });
        alert('Mappings saved to profile');
    } catch (error) {
        console.error('Failed to save mappings:', error);
        alert('Failed to save mappings');
    }
}

// Profiles
async function loadProfiles() {
    try {
        const profiles = await invoke('load_all_profiles');
        state.profiles = profiles;
        populateGameFilter();
    } catch (error) {
        console.error('Failed to load profiles:', error);
    }
}

function renderProfiles(profiles = state.profiles) {
    if (profiles.length === 0) {
        elements.profilesList.innerHTML = `
            <div class="empty-state">
                <p>No profiles found</p>
                <p class="text-muted">Create a profile to get started</p>
            </div>
        `;
        return;
    }
    
    elements.profilesList.innerHTML = profiles.map(profile => `
        <div class="profile-card" data-id="${profile.id}" onclick="selectProfile('${profile.id}')">
            <h3>${profile.name}</h3>
            <p>${profile.game_name}</p>
            <p>${profile.description || 'No description'}</p>
            <div class="profile-meta">
                <span>By ${profile.author}</span>
                <span>${new Date(profile.updated_at).toLocaleDateString()}</span>
            </div>
            <div class="profile-actions">
                <button class="btn btn-primary btn-sm" onclick="event.stopPropagation(); loadProfile('${profile.id}')">Load</button>
                <button class="btn btn-secondary btn-sm" onclick="event.stopPropagation(); duplicateProfile('${profile.id}')">Duplicate</button>
                <button class="btn btn-icon" onclick="event.stopPropagation(); deleteProfile('${profile.id}')">
                    <svg width="16" height="16" viewBox="0 0 16 16" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M4 4l8 8M4 12L12 4"/>
                    </svg>
                </button>
            </div>
        </div>
    `).join('');
}

function populateGameFilter() {
    const games = [...new Set(state.profiles.map(p => p.game_name))];
    elements.filterGame.innerHTML = '<option value="">All Games</option>' +
        games.map(game => `<option value="${game}">${game}</option>`).join('');
    
    elements.filterGame.addEventListener('change', (e) => {
        if (e.target.value) {
            const filtered = state.profiles.filter(p => p.game_name === e.target.value);
            renderProfiles(filtered);
        } else {
            renderProfiles();
        }
    });
}

function handleSearchProfiles(event) {
    const query = event.target.value.toLowerCase();
    const filtered = state.profiles.filter(p => 
        p.name.toLowerCase().includes(query) ||
        p.game_name.toLowerCase().includes(query) ||
        p.description.toLowerCase().includes(query)
    );
    renderProfiles(filtered);
}

async function selectProfile(profileId) {
    try {
        state.selectedProfile = state.profiles.find(p => p.id === profileId);
        
        // Highlight selected profile
        document.querySelectorAll('.profile-card').forEach(card => {
            card.classList.toggle('selected', card.dataset.id === profileId);
        });
        
        // Load mappings if controller is selected
        if (state.selectedController) {
            await loadProfileMappings(profileId);
        }
    } catch (error) {
        console.error('Failed to select profile:', error);
    }
}

async function loadProfileMappings(profileId) {
    try {
        const profile = await invoke('get_profile', { profileId });
        state.mappings = profile.button_mappings.map(m => ({
            button: m.controller_button,
            key: m.keyboard_key
        }));
        renderMappings();
    } catch (error) {
        console.error('Failed to load profile mappings:', error);
    }
}

async function loadProfile(profileId) {
    try {
        const profile = await invoke('get_profile', { profileId });
        state.selectedProfile = profile;
        state.mappings = profile.button_mappings.map(m => ({
            button: m.controller_button,
            key: m.keyboard_key
        }));
        
        // Apply mappings
        const mapper = await invoke('get_input_mapper');
        await invoke('set_input_mapper', { mapper });
        
        alert(`Profile "${profile.name}" loaded`);
    } catch (error) {
        console.error('Failed to load profile:', error);
        alert('Failed to load profile');
    }
}

function showCreateProfileModal() {
    elements.modalTitle.textContent = 'Create Profile';
    elements.modalBody.innerHTML = `
        <form id="createProfileForm">
            <div class="form-group">
                <label for="profileName">Profile Name</label>
                <input type="text" id="profileName" required>
            </div>
            <div class="form-group">
                <label for="gameName">Game Name</label>
                <input type="text" id="gameName" required>
            </div>
            <div class="form-group">
                <label for="author">Author</label>
                <input type="text" id="author" required>
            </div>
            <div class="form-group">
                <label for="description">Description</label>
                <textarea id="description"></textarea>
            </div>
            <div class="modal-footer">
                <button type="button" class="btn btn-secondary" onclick="closeModal()">Cancel</button>
                <button type="submit" class="btn btn-primary">Create</button>
            </div>
        </form>
    `;
    
    elements.modal.classList.remove('hidden');
    
    document.getElementById('createProfileForm').addEventListener('submit', async (e) => {
        e.preventDefault();
        
        try {
            const profile = await invoke('create_profile', {
                name: document.getElementById('profileName').value,
                gameName: document.getElementById('gameName').value,
                author: document.getElementById('author').value
            });
            
            profile.description = document.getElementById('description').value;
            await invoke('save_profile', { profile });
            
            await loadProfiles();
            renderProfiles();
            closeModal();
        } catch (error) {
            console.error('Failed to create profile:', error);
            alert('Failed to create profile');
        }
    });
}

async function duplicateProfile(profileId) {
    try {
        const profile = state.profiles.find(p => p.id === profileId);
        const newName = prompt('Enter name for duplicated profile:', `${profile.name} (Copy)`);
        
        if (newName) {
            await invoke('duplicate_profile', { profileId, newName: newName });
            await loadProfiles();
            renderProfiles();
        }
    } catch (error) {
        console.error('Failed to duplicate profile:', error);
        alert('Failed to duplicate profile');
    }
}

async function deleteProfile(profileId) {
    if (confirm('Are you sure you want to delete this profile?')) {
        try {
            await invoke('delete_profile', { profileId });
            await loadProfiles();
            renderProfiles();
            
            if (state.selectedProfile?.id === profileId) {
                state.selectedProfile = null;
            }
        } catch (error) {
            console.error('Failed to delete profile:', error);
            alert('Failed to delete profile');
        }
    }
}

// Workshop
function switchWorkshopTab(tab) {
    elements.workshopTabs.forEach(t => {
        t.classList.toggle('active', t.dataset.tab === tab);
    });
    
    if (tab === 'subscribed') {
        loadSubscribedItems();
    } else if (tab === 'my-items') {
        loadMyWorkshopItems();
    } else {
        elements.workshopContent.innerHTML = `
            <div class="empty-state">
                <p>Search for profiles in the Steam Workshop</p>
            </div>
        `;
    }
}

async function searchWorkshop() {
    const query = elements.searchWorkshop.value;
    
    try {
        const items = await invoke('search_workshop', { query, tags: [] });
        renderWorkshopItems(items);
    } catch (error) {
        console.error('Failed to search workshop:', error);
    }
}

function renderWorkshopItems(items) {
    if (items.length === 0) {
        elements.workshopContent.innerHTML = `
            <div class="empty-state">
                <p>No items found</p>
            </div>
        `;
        return;
    }
    
    elements.workshopContent.innerHTML = items.map(item => `
        <div class="workshop-item">
            <h3>${item.title}</h3>
            <p>${item.description}</p>
            <div class="workshop-stats">
                <span>⭐ ${item.upvotes}</span>
                <span>📥 ${item.subscriptions}</span>
            </div>
            <div class="workshop-actions">
                <button class="btn btn-primary" onclick="subscribeWorkshopItem(${item.workshop_id})">Subscribe</button>
                <button class="btn btn-secondary" onclick="downloadWorkshopItem(${item.workshop_id})">Download</button>
            </div>
        </div>
    `).join('');
}

async function subscribeWorkshopItem(workshopId) {
    try {
        await invoke('subscribe_workshop_item', { workshopId });
        alert('Subscribed to item');
    } catch (error) {
        console.error('Failed to subscribe:', error);
        alert('Failed to subscribe');
    }
}

async function downloadWorkshopItem(workshopId) {
    try {
        const profile = await invoke('download_from_workshop', { workshopId });
        await invoke('save_profile', { profile });
        await loadProfiles();
        renderProfiles();
        alert('Profile downloaded');
    } catch (error) {
        console.error('Failed to download:', error);
        alert('Failed to download profile');
    }
}

async function loadSubscribedItems() {
    try {
        const items = await invoke('get_subscribed_items');
        renderWorkshopItems(items);
    } catch (error) {
        console.error('Failed to load subscribed items:', error);
    }
}

async function loadMyWorkshopItems() {
    elements.workshopContent.innerHTML = `
        <div class="empty-state">
            <p>My workshop items feature coming soon</p>
        </div>
    `;
}

// Cloud
async function loadCloudStatus() {
    try {
        const quota = await invoke('get_cloud_quota');
        elements.cloudQuota.textContent = `${formatBytes(quota[1])} / ${formatBytes(quota[0])}`;
        elements.cloudStatus.textContent = 'Connected';
        elements.cloudStatus.classList.add('text-success');
        
        await loadCloudProfiles();
    } catch (error) {
        console.error('Failed to load cloud status:', error);
        elements.cloudStatus.textContent = 'Disconnected';
        elements.cloudStatus.classList.add('text-error');
    }
}

async function loadCloudProfiles() {
    try {
        const profileIds = await invoke('list_cloud_profiles');
        
        if (profileIds.length === 0) {
            elements.cloudProfilesList.innerHTML = `
                <div class="empty-state">
                    <p>No profiles in cloud</p>
                </div>
            `;
            return;
        }
        
        const profiles = await Promise.all(
            profileIds.map(id => invoke('load_profile_from_cloud', { profileId: id }))
        );
        
        elements.cloudProfilesList.innerHTML = profiles.map(profile => `
            <div class="profile-card">
                <h3>${profile.name}</h3>
                <p>${profile.game_name}</p>
                <div class="profile-actions">
                    <button class="btn btn-primary" onclick="downloadCloudProfile('${profile.id}')">Download</button>
                </div>
            </div>
        `).join('');
    } catch (error) {
        console.error('Failed to load cloud profiles:', error);
    }
}

async function syncCloud() {
    try {
        await invoke('sync_cloud');
        alert('Cloud sync completed');
        await loadCloudStatus();
    } catch (error) {
        console.error('Failed to sync cloud:', error);
        alert('Failed to sync cloud');
    }
}

async function downloadCloudProfile(profileId) {
    try {
        const profile = await invoke('load_profile_from_cloud', { profileId });
        await invoke('save_profile', { profile });
        await loadProfiles();
        renderProfiles();
        alert('Profile downloaded from cloud');
    } catch (error) {
        console.error('Failed to download profile:', error);
        alert('Failed to download profile');
    }
}

// Utility Functions
function closeModal() {
    elements.modal.classList.add('hidden');
}

function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

function formatBytes(bytes) {
    if (bytes === 0) return '0 Bytes';
    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
}

// Initialize on load
document.addEventListener('DOMContentLoaded', init);
