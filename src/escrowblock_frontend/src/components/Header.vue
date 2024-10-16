<script>
</script>

<template>
	<header id="topnav" class="defaultscroll sticky">
        <div class="container">
            <a class="logo" href="index.html">
                    <span class="logo-light-mode">
                        <img src="/public/images/logo-dark.png" class="l-dark" alt="">
                        <img src="/public/images/logo-white.png" class="l-light" alt="">
                    </span>
                    <img src="/public/images/logo-white.png" class="logo-dark-mode" alt="">
                </a>

            <div class="menu-extras">
                <div class="menu-item">
                    <a class="navbar-toggle" id="isToggle" onclick="toggleMenu()">
                        <div class="lines">
                            <span></span>
                            <span></span>
                            <span></span>
                        </div>
                    </a>
                </div>
            </div>

            <ul class="buy-button list-inline mb-0">
                <li class="list-inline-item mb-0">
                    <div class="dropdown">
                        <button type="button" class="dropdown-toggle btn btn-sm p-0 border-0" data-bs-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                                <i data-feather="search" class="fea icon-20 login-btn-primary"></i>
                                <i data-feather="search" class="fea icon-20 text-white login-btn-light"></i>
                            </button>
                        <div class="dropdown-menu dd-menu dropdown-menu-end bg-white rounded border-0 mt-3 p-0" style="width: 240px;">
                            <div class="search-bar">
                                <div id="itemSearch" class="menu-search mb-0">
                                    <form role="search" method="get" id="searchItemform" class="searchform">
                                        <input type="text" class="form-control rounded border" name="s" id="searchItem" placeholder="Search...">
                                        <input type="submit" id="searchItemsubmit" value="Search">
                                        <i data-feather="search" class="icons"></i>
                                    </form>
                                </div>
                            </div>
                        </div>
                    </div>
                </li>

                <li class="list-inline-item ps-1 mb-0">
                    <a href="javascript:void(0)" data-bs-toggle="offcanvas" data-bs-target="#offcanvasRight" aria-controls="offcanvasRight">
                        <div class="btn btn-sm btn-icon btn-pills btn-primary"><i data-feather="settings" class="fea icon-sm"></i></div>
                    </a>
                </li>

                <li class="list-inline-item ps-1 mb-0">
                    <a href="" class="btn btn-sm btn-primary">Sign up</a>
                </li>
            </ul>

            <div id="navigation">
                <!-- Navigation Menu-->
                <ul class="navigation-menu nav-right nav-light" id="navmenu-nav">
                    <li class="has-submenu">
                        <a href="#home">Home</a>
                    </li>
                    <li class="has-submenu">
                        <a href="#cryptocurrency">Cryptocurrency</a>
                    </li>
                    <li class="has-submenu">
                        <a href="#trader">Become a Trader</a>
                    </li>
                    <li class="has-submenu">
                        <a href="#testimonial">Testimonial</a>
                    </li>
                    <li class="has-submenu">
                        <a href="#application">Application</a>
                    </li>
                </ul>
                <!--end navigation menu-->
            </div>
            <!--end navigation-->
        </div>
    </header>
</template>