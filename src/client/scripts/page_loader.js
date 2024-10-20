function page_loader() {
    return {
        currentSlide: 2,
        loadedContent: '', // Holds the currently loaded slide content
        slides: [
            './pages/events.html', // Path to the first slide
            './pages/permanent-profile.html', // Path to the second slide
            './pages/temporary-profile.html'  // Path to the third slide
        ],
        // Preload the next slide
        preloadNextSlide() {
            const nextSlide = (this.currentSlide + 1) % this.slides.length;
            fetch(this.slides[nextSlide]); // Preload the next slide in advance
        },
        // Load the content of the current slide
        loadSlide() {
            fetch(this.slides[this.currentSlide])
                .then(response => response.text())
                .then(html => {
                    this.loadedContent = html; // Load the content into the carousel
                    this.preloadNextSlide(); // Preload the next slide
                })
                .catch(error => console.error('Error loading slide:', error));
        },
        nextSlide() {
            this.currentSlide = (this.currentSlide + 1) % this.slides.length;
            this.loadSlide(); // Load the next slide
        },
        prevSlide() {
            this.currentSlide = (this.currentSlide - 1 + this.slides.length) % this.slides.length;
            this.loadSlide(); // Load the previous slide
        },
        // Initial load of the first slide
        init() {
            this.loadSlide();
        }
    }
}
