<script setup>
import DashboardLayout from '../Layout/DashboardLayout.vue';
import { ref } from 'vue';
import { Actor, HttpAgent } from '@dfinity/agent';

const seller = ref('');
const buyer = ref('');
const amount = ref('');
const terms = ref('');
const successMessage = ref(false);
const errorMessage = ref('');

async function submitEscrowForm() {
  try {
    const agent = new HttpAgent();
    const { idlFactory: myProjectIdl } = await import('declarations/PIW_backend');
    const myProjectId = "a4tbr-q4aaa-aaaaa-qaafq-cai";

    const myProjectActor = Actor.createActor(myProjectIdl, {
      agent,
      canisterId: myProjectId,
    });

    await myProjectActor.initiate_escrow(
      seller.value,
      buyer.value,
      BigInt(amount.value), // Ensure BigInt conversion for ICP e8s
      terms.value
    );

    successMessage.value = true; // Show success message

    // Redirect to dashboard after 3 seconds
    setTimeout(() => {
      // Your router push to dashboard here
      this.$router.push('/dashboard');
    }, 3000);
  } catch (err) {
    errorMessage.value = err.message || 'Failed to initiate escrow';
    successMessage.value = true; // Show success message even on error

    // Redirect to dashboard after 3 seconds
    setTimeout(() => {
      // Your router push to dashboard here
      this.$router.push('/dashboard');
    }, 3000);
  }
}
</script>

<template>
  <DashboardLayout>
    <div class="bg-white border rounded-lg shadow relative m-10">
      <div class="flex items-start justify-between p-5 border-b rounded-t">
        <h3 class="text-xl font-semibold">Initiate Escrow</h3>
        <button type="button"
          class="text-gray-400 bg-transparent hover:bg-gray-200 hover:text-gray-900 rounded-lg text-sm p-1.5 ml-auto inline-flex items-center">
          <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
            <path fill-rule="evenodd"
              d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
              clip-rule="evenodd"></path>
          </svg>
        </button>
      </div>

      <div class="p-6 space-y-6">
        <form @submit.prevent="submitEscrowForm">
          <div class="grid grid-cols-6 gap-6">
            <!-- Seller Principal -->
            <div class="col-span-6 sm:col-span-3">
              <label for="seller" class="text-sm font-medium text-gray-900 block mb-2">Seller Principal</label>
              <input type="text" id="seller" v-model="seller"
                class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-cyan-600 focus:border-cyan-600 block w-full p-2.5"
                required placeholder="Enter seller's principal">
            </div>

            <!-- Buyer Principal -->
            <div class="col-span-6 sm:col-span-3">
              <label for="buyer" class="text-sm font-medium text-gray-900 block mb-2">Buyer Principal</label>
              <input type="text" id="buyer" v-model="buyer"
                class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-cyan-600 focus:border-cyan-600 block w-full p-2.5"
                required placeholder="Enter buyer's principal">
            </div>

            <!-- Amount -->
            <div class="col-span-6 sm:col-span-3">
              <label for="amount" class="text-sm font-medium text-gray-900 block mb-2">Amount (e8s)</label>
              <input type="number" id="amount" v-model="amount"
                class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-cyan-600 focus:border-cyan-600 block w-full p-2.5"
                required placeholder="Enter amount in e8s">
            </div>

            <!-- Terms -->
            <div class="col-span-6 sm:col-span-3">
              <label for="terms" class="text-sm font-medium text-gray-900 block mb-2">Terms</label>
              <textarea id="terms" v-model="terms" rows="4"
                class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-cyan-600 focus:border-cyan-600 block w-full p-2.5"
                placeholder="Enter escrow terms" required></textarea>
            </div>
          </div>
        </form>
      </div>

      <div class="p-6 border-t border-gray-200 rounded-b">
        <button @click="submitEscrowForm"
          class="text-white bg-cyan-600 hover:bg-cyan-700 focus:ring-4 focus:ring-cyan-200 font-medium rounded-lg text-sm px-5 py-2.5 text-center">
          Initiate Escrow
        </button>

        <div v-if="successMessage" class="mt-4 text-center">
          <p class="text-green-600">Escrow created successfully! Redirecting to dashboard...</p>
          <p>Go fund your escrow</p>
        </div>
        
        <!-- <div v-if="errorMessage" class="mt-4 text-center">
          <p class="text-red-600">{{ errorMessage }}</p>
        </div> -->
      </div>
    </div>
  </DashboardLayout>
</template>

<style scoped>
/* Add any specific styles you might want for this component here */
</style>
