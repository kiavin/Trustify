<script setup>
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { AuthClient } from '@dfinity/auth-client';

const router = useRouter();

const loginWithInternetIdentity = async () => {
  console.log('Starting Internet Identity login'); // Debug log
  const authClient = await AuthClient.create();
  await authClient.login({
    identityProvider: `https://identity.ic0.app/?canisterId=${import.meta.env.VITE_CANISTER_ID_INTERNET_IDENTITY}`,
    onSuccess: () => {
      console.log('Login successful'); // Debug log
      // Redirect to /dashboard
      console.log('Redirecting to /dashboard'); // Debug log
      router.push('/dashboard');
    },
    onError: (error) => {
      console.error('Login failed', error); // Debug log
    },
  });
};

const loginWithGoogle = async () => {
  console.log('Starting Google login'); // Debug log
  // Simulate Google login success
  console.log('Google login successful'); // Debug log
  // Redirect to /dashboard
  console.log('Redirecting to /dashboard'); // Debug log
  router.push('/dashboard');
};
</script>

<template>
  <!-- Registration 8 - Bootstrap Brain Component -->
  <section class="bg-light p-3 p-md-4 p-xl-5">
    <div class="container">
      <div class="row justify-content-center">
        <div class="col-12 col-xxl-11">
          <div class="card border-light-subtle shadow-sm">
            <div class="row g-0">
              <div class="col-12 col-md-6">
                <img class="img-fluid rounded-start w-100 h-100 object-fit-cover" loading="lazy" src="/login.jpg" alt="Welcome back you've been missed!">
              </div>
              <div class="col-12 col-md-6 d-flex align-items-center justify-content-center">
                <div class="col-12 col-lg-11 col-xl-10">
                  <div class="card-body p-3 p-md-4 p-xl-5">
                    <div class="row">
                      <div class="col-12">
                        <div class="mb-5">
                          <div class="text-center mb-4">
                            <a href="#!">
                              <img src="/logo.png" alt="BootstrapBrain Logo" width="175" height="57">
                            </a>
                          </div>
                          <h2 class="h4 text-center">Registration</h2>
                        </div>
                      </div>
                    </div>
                    <div class="row">
                      <div class="col-12">
                        <div class="d-flex gap-3 flex-column">
                          <button @click="loginWithInternetIdentity" class="btn btn-lg btn-outline-dark">
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-shield" viewBox="0 0 16 16">
                              <path d="M5.072 1.342a1.999 1.999 0 0 1 1.856 0l4.9 2.45a2 2 0 0 1 1.1 1.789v3.511c0 2.43-1.397 4.71-3.48 5.792L8 15.528l-1.448-.646C4.47 13.803 3.072 11.523 3.072 9.092V5.581a2 2 0 0 1 1.1-1.79l4.9-2.45zM8 .5a3 3 0 0 0-1.856.678L1.243 3.577A3 3 0 0 0 0 6.196v3.511c0 3.07 1.97 5.939 5.29 7.385L8 17l2.71-1.209C14.03 15.646 16 12.777 16 9.707V6.196a3 3 0 0 0-1.243-2.619L9.856 1.178A3 3 0 0 0 8 .5z"/>
                            </svg>
                            <span class="ms-2 fs-6">Log in with Internet Identity</span>
                          </button>
                        </div>
                        <div class="d-flex gap-3 flex-column">
                          <button @click="loginWithGoogle" class="btn btn-lg btn-outline-dark">
                            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-globe" viewBox="0 0 16 16">
                              <path d="M15.545 6.558a9.42 9.42 0 0 1 .139 1.626c0 2.434-.87 4.492-2.384 5.885h.002C11.978 15.292 10.158 16 8 16A8 8 0 1 1 8 0a7.689 7.689 0 0 1 5.352 2.082l-2.284 2.284A4.347 4.347 0 0 0 8 3.166c-2.087 0-3.86 1.408-4.492 3.304a4.792 4.792 0 0 0 0 3.063h.003c.635 1.893 2.405 3.301 4.492 3.301 1.078 0 2.004-.276 2.722-.764h-.003a3.702 3.702 0 0 0 1.599-2.431H8v-3.08h7.545z" />
                            </svg>
                            <span class="ms-2 fs-6">Log in with Google</span>
                          </button>
                        </div>
                        <p class="text-center mt-4 mb-5">Or enter your details to register</p>
                      </div>
                    </div>
                    <form action="#!">
                      <div class="row gy-3 overflow-hidden">
                        <div class="col-12">
                          <div class="form-floating mb-3">
                            <input type="text" class="form-control" name="firstName" id="firstName" placeholder="First Name" required>
                            <label for="firstName" class="form-label">First Name</label>
                          </div>
                        </div>
                        <div class="col-12">
                          <div class="form-floating mb-3">
                            <input type="text" class="form-control" name="lastName" id="lastName" placeholder="First Name" required>
                            <label for="lastName" class="form-label">Last Name</label>
                          </div>
                        </div>
                        <div class="col-12">
                          <div class="form-floating mb-3">
                            <input type="email" class="form-control" name="email" id="email" placeholder="name@example.com" required>
                            <label for="email" class="form-label">Email</label>
                          </div>
                        </div>
                        <div class="col-12">
                          <div class="form-floating mb-3">
                            <input type="password" class="form-control" name="password" id="password" value="" placeholder="Password" required>
                            <label for="password" class="form-label">Password</label>
                          </div>
                        </div>
                        <div class="col-12">
                          <div class="form-check">
                            <input class="form-check-input" type="checkbox" value="" name="iAgree" id="iAgree" required>
                            <label class="form-check-label text-secondary" for="iAgree">
                              I agree to the <a href="#!" class="link-primary text-decoration-none">terms and conditions</a>
                            </label>
                          </div>
                        </div>
                        <div class="col-12">
                          <div class="d-grid">
                            <button class="btn btn-dark btn-lg" type="submit">Sign up</button>
                          </div>
                        </div>
                      </div>
                    </form>
                    <div class="row">
                      <div class="col-12">
                        <p class="mb-0 mt-5 text-secondary text-center">Already have an account?
                          <a href="#!" class="link-primary text-decoration-none">Sign in</a>
                        </p>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>