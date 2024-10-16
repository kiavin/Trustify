<script setup>
// No specific JavaScript needed for basic button routing unless you want to add functionality
</script>

<template>
<nav class="bg-white border-gray-200 py-2.5 dark:bg-purple-800">
    <div class="flex flex-wrap items-center justify-between max-w-screen-xl px-4 mx-auto">
        <RouterLink to="/" class="flex items-center">
            <img src="/logo.png" class="h-6 mr-3 sm:h-9" alt="Landwind Logo">
            <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-purple-700">Trustify</span>
        </RouterLink>
        
        <!-- Right-side buttons for Login and Register -->
        <div class="flex items-center lg:order-2">
            <RouterLink to="/auth/Login" class="text-purple-700 hover:bg-purple-800 hover:text-white px-4 py-2 border border-purple-700 rounded-lg mr-2  dark:text-purple-700 dark:border-white dark:hover:bg-purple-700 dark:hover:border-purple-600">
                Login
            </RouterLink>
            <RouterLink to="/auth/Register" class="text-white bg-purple-700 hover:bg-purple-600 px-4 py-2 border border-purple-700 rounded-lg dark:bg-purple-600 dark:hover:bg-purple-700">
                Register
            </RouterLink>
            
            <!-- Mobile Menu Toggle Button -->
            <button data-collapse-toggle="mobile-menu-2" type="button"
				class="inline-flex items-center p-2 ml-1 text-sm text-purple-700 rounded-lg lg:hidden hover:bg-purple-400 focus:outline-none focus:ring-2 focus:ring-gray-200 dark:text-gray-400 dark:hover:bg-gray-700 dark:focus:ring-gray-600"
				aria-controls="mobile-menu-2" aria-expanded="true">
				<span class="sr-only">Open main menu</span>
				<svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
					<path fill-rule="evenodd"
						d="M3 5a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 10a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zM3 15a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1z"
						clip-rule="evenodd"></path>
				</svg>
				<svg class="hidden w-6 h-6" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
					<path fill-rule="evenodd"
						d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
						clip-rule="evenodd"></path>
				</svg>
			</button>
        </div>

        <div class="items-center justify-between w-full lg:flex lg:w-auto lg:order-1" id="mobile-menu-2">
            <ul class="flex flex-col mt-4 font-medium lg:flex-row lg:space-x-8 lg:mt-0">
                <li>
                    <RouterLink to="/"
                        class="block py-2 pl-3 pr-4 text-purple-700 border-b border-gray-100 hover:bg-purple-800 lg:hover:bg-purple-700 lg:border-0 lg:hover:text-white lg:p-0 dark:text-purple-700 lg:dark:hover:text-white dark:hover:bg-purple-800 dark:hover:text-white lg:dark:hover:bg-purple-700 dark:border-gray-700">Home</RouterLink>
                </li>
                <li>
                    <RouterLink to="/About"
                        class="block py-2 pl-3 pr-4 text-purple-700 border-b border-gray-100 hover:bg-purple-800 lg:hover:bg-purple-700 lg:border-0 lg:hover:text-white lg:p-0 dark:text-purple-700 lg:dark:hover:text-white dark:hover:bg-purple-800 dark:hover:text-white lg:dark:hover:bg-purple-700 dark:border-gray-700">About Us</RouterLink>
                </li>
                <li>
                    <RouterLink to="/Contact"
                        class="block py-2 pl-3 pr-4 text-purple-700 border-b border-gray-100 hover:bg-purple-800 lg:hover:bg-purple-700 lg:border-0 lg:hover:text-white lg:p-0 dark:text-purple-700 lg:dark:hover:text-white dark:hover:bg-purple-800 dark:hover:text-white lg:dark:hover:bg-purple-700 dark:border-gray-700">Contact</RouterLink>
                </li>
                <li>
                    <RouterLink to="/Features"
                        class="block py-2 pl-3 pr-4 text-purple-700 border-b border-gray-100 hover:bg-purple-800 lg:hover:bg-purple-700 lg:border-0 lg:hover:text-white lg:p-0 dark:text-purple-700 lg:dark:hover:text-white dark:hover:bg-purple-800 dark:hover:text-white lg:dark:hover:bg-purple-700 dark:border-gray-700">Features</RouterLink>
                </li>
            </ul>
        </div>
    </div>
</nav>
</template>
