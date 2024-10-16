<script setup>
import DashboardLayout from '../Layout/DashboardLayout.vue';
import { ref } from 'vue';

const transactionTitle = ref('');
const fullName = ref('');
const email = ref('');
const industryType = ref('');
const dueDate = ref('');
const amount = ref('');
const vendorId = ref('');
const milestones = ref('');

function submitEscrowForm() {
    const newEscrowData = {
        transactionTitle: transactionTitle.value,
        fullName: fullName.value,
        email: email.value,
        industryType: industryType.value,
        dueDate: dueDate.value,
        amount: amount.value,
        vendorId: vendorId.value,
        milestones: milestones.value,
    };
    console.log('New Escrow Created:', newEscrowData);
    // Add logic to send `newEscrowData` to an API or handle it further
}
</script>

<template>
    <DashboardLayout>
    <div class="bg-white border  rounded-lg shadow relative m-10">
        <div class="flex items-start justify-between p-5 border-b rounded-t">
            <h3 class="text-xl font-semibold">Create New Escrow</h3>
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
                    <!-- Transaction Title -->
                    <div class="col-span-6 sm:col-span-3">
                        <label for="transactionTitle" class="text-sm font-medium text-gray-900 block mb-2">Transaction
                            Title</label>
                        <input type="text" id="transactionTitle" v-model="transactionTitle"
                            class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-cyan-600 focus:border-cyan-600 block w-full p-2.5"
                            required>
                    </div>

                    <!-- Full Name -->
                    <div class="col-span-6 sm:col-span-3">
                        <label for="fullName" class="text-sm font-medium text-gray-900 block mb-2">Full Name</label>
                        <input type="text" id="fullName" v-model="fullName"
                            class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-cyan-600 focus:border-cyan-600 block w-full p-2.5"
                            required>
                    </div>

                    <!-- Email -->
                    <div class="col-span-6 sm:col-span-3">
                        <label for="email" class="text-sm font-medium text-gray-900 block mb-2">Email</label>
                        <input type="email" id="email" v-model="email"
                            class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-cyan-600 focus:border-cyan-600 block w-full p-2.5"
                            required>
                    </div>

                    <!-- Industry Type -->
                    <div class="col-span-6 sm:col-span-3">
                        <label for="industryType" class="text-sm font-medium text-gray-900 block mb-2">Industry
                            Type</label>
                        <input type="text" id="industryType" v-model="industryType"
                            class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-cyan-600 focus:border-cyan-600 block w-full p-2.5"
                            required>
                    </div>

                    <!-- Due Date -->
                    <div class="col-span-6 sm:col-span-3">
                        <label for="dueDate" class="text-sm font-medium text-gray-900 block mb-2">Due Date</label>
                        <input type="date" id="dueDate" v-model="dueDate"
                            class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-cyan-600 focus:border-cyan-600 block w-full p-2.5"
                            required>
                    </div>

                    <!-- Amount -->
                    <div class="col-span-6 sm:col-span-3">
                        <label for="amount" class="text-sm font-medium text-gray-900 block mb-2">Amount</label>
                        <input type="number" id="amount" v-model="amount"
                            class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-cyan-600 focus:border-cyan-600 block w-full p-2.5"
                            required>
                    </div>

                    <!-- Vendor ID -->
                    <div class="col-span-6 sm:col-span-3">
                        <label for="vendorId" class="text-sm font-medium text-gray-900 block mb-2">Vendor ID</label>
                        <input type="text" id="vendorId" v-model="vendorId"
                            class="shadow-sm bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-cyan-600 focus:border-cyan-600 block w-full p-2.5"
                            required>
                    </div>

                    <!-- Milestones -->
                    <div class="col-span-full">
                        <label for="milestones" class="text-sm font-medium text-gray-900 block mb-2">Milestones</label>
                        <textarea id="milestones" v-model="milestones" rows="4"
                            class="bg-gray-50 border border-gray-300 text-gray-900 sm:text-sm rounded-lg focus:ring-cyan-600 focus:border-cyan-600 block w-full p-2.5"
                            placeholder="Enter milestones"></textarea>
                    </div>
                </div>
            </form>
        </div>

        <div class="p-6 border-t border-gray-200 rounded-b">
            <button @click="submitEscrowForm"
                class="text-white bg-cyan-600 hover:bg-cyan-700 focus:ring-4 focus:ring-cyan-200 font-medium rounded-lg text-sm px-5 py-2.5 text-center">Create
                Escrow</button>
        </div>
    </div>
</DashboardLayout>
</template>
