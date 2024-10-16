<script setup>
// You can define the header text here if you want to dynamically bind it
const headers = [
    { text: "Secure. Transparent. Reliable Escrow on Blockchain.", subtext: "Trust your transactions with decentralized escrow protection." },
    { text: "Empowering Trust with Blockchain Escrow Services", subtext: "Transact securely with the power of smart contracts." },
    { text: "Eliminate Risk with Blockchain-Powered Escrow", subtext: "Your funds, protected by technology you can trust." }
];
</script>

<template>
    <div id="carouselExampleIndicators" class="carousel slide" data-bs-ride="carousel">
        <div class="carousel-indicators">
            <button type="button" data-bs-target="#carouselExampleIndicators" data-bs-slide-to="0" class="active"
                aria-current="true" aria-label="Slide 1"></button>
            <button type="button" data-bs-target="#carouselExampleIndicators" data-bs-slide-to="1"
                aria-label="Slide 2"></button>
            <button type="button" data-bs-target="#carouselExampleIndicators" data-bs-slide-to="2"
                aria-label="Slide 3"></button>
        </div>
        <div class="carousel-inner">
            <div class="carousel-item active">
                <img src="/cr1.png" class="d-block w-100" alt="slide1">
                <div class="carousel-caption">
                    <h1>{{ headers[0].text }}</h1>
                    <p>{{ headers[0].subtext }}</p>
                </div>
            </div>
            <div class="carousel-item">
                <img src="/cr1.png" class="d-block w-100" alt="slide2">
                <div class="carousel-caption">
                    <h1>{{ headers[1].text }}</h1>
                    <p>{{ headers[1].subtext }}</p>
                </div>
            </div>
            <div class="carousel-item">
                <img src="/cr1.png" class="d-block w-100" alt="slide3">
                <div class="carousel-caption">
                    <h1>{{ headers[2].text }}</h1>
                    <p>{{ headers[2].subtext }}</p>
                </div>
            </div>
        </div>
        <button class="carousel-control-prev" type="button" data-bs-target="#carouselExampleIndicators"
            data-bs-slide="prev">
            <span class="carousel-control-prev-icon" aria-hidden="true"></span>
            <span class="visually-hidden">Previous</span>
        </button>
        <button class="carousel-control-next" type="button" data-bs-target="#carouselExampleIndicators"
            data-bs-slide="next">
            <span class="carousel-control-next-icon" aria-hidden="true"></span>
            <span class="visually-hidden">Next</span>
        </button>
    </div>
</template>

<style>
.carousel-caption {
    position: absolute;
    top: 40%;

    transform: translate(-50%, -50%);
    color: #fff;
    text-align: center;
    padding: 10px;
    border-radius: 8px;
    max-width: 100%;
    left:30% !important;
    right: 0 !important;
}

.carousel-caption h1 {
    font-size: 4rem; /* Adjust font size for the header */
    font-weight: bold;
    font-family: 'Arial', sans-serif; /* Change font to your preferred style */
    color:#fff;
}

.carousel-caption p {
    font-size: 1.5rem; /* Adjust font size for the subtext */
    font-family: 'Arial', sans-serif;
    margin-top: 10px;
}
</style>
