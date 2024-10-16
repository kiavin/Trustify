<script setup></script>
<template>
    <div class="mt-8 grid gap-8 sm:grid-cols-2 lg:grid-cols-3 p-4 md:p-8">

        <!-- Card 1 -->
        <div
            class="p-4 rounded-lg mycards border border-gray-100 shadow-md hover:shadow-lg transition-shadow duration-300">
            <div class="flex flex-col items-center space-y-4">
                <!-- Image -->
                <img class="w-16 h-16 object-cover rounded-full" src="/eth-dynamic-color.png" alt="Data Collection">

                <!-- Title -->
                <h3 class="text-lg font-semibold text-gray-800 text-center">Wallet</h3>
            </div>
        </div>

        <!-- Card 2 -->
        <div
            class="p-4 rounded-lg mycards border border-gray-100 shadow-md hover:shadow-lg transition-shadow duration-300">
            <div class="flex flex-col items-center space-y-4">
                <!-- Image -->
                <img class="w-16 h-16 object-cover rounded-full" src="/card-dynamic-color.png" alt="Model Training">

                <!-- Title -->
                <h3 class="text-lg font-semibold text-gray-800 text-center">Buy Tokens</h3>
            </div>
        </div>

        <!-- Card 3 -->
        <div
            class="p-4 rounded-lg mycards border border-gray-100 shadow-md hover:shadow-lg transition-shadow duration-300">
            <div class="flex flex-col items-center space-y-4">
                <!-- Image -->
                <img class="w-16 h-16 object-cover rounded-full" src="/locker-dynamic-color.png" alt="Insights Delivery">

                <!-- Title -->
                <h3 class="text-lg font-semibold text-gray-800 text-center">Pending Transactions</h3>
            </div>
        </div>

    </div>
</template>
<style scoped>
.mycards {
    background: linear-gradient(to bottom right, #562CBA, #E4B161);
}
</style>
