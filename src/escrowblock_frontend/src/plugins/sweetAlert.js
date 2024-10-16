import Swal from 'sweetalert2';

export default {
  install: (app, options) => {
    // General alert function
    app.config.globalProperties.$swal = ({
      title = 'Default Title',
      text = 'Default Text',
      icon = 'success',
      showConfirmButton = false,
      showLoaderOnConfirm = false,
      options = {}
    }) => {
      return Swal.fire({
        title,
        text,
        icon,
        showConfirmButton,
        showLoaderOnConfirm,
        ...options, // Spread the options for additional customizations
      });
    };

    // Toast function
    app.config.globalProperties.$addToast = ({
      title = 'Default Title',
      text = '',
      icon = 'success',
      position = 'top-end',
      showConfirmButton = false,
      timerProgressBar = true,
      options = {}
    }) => {
      return Swal.fire({
        toast: true,
        position,
        showConfirmButton,
        timer: 3000,
        timerProgressBar,
        title,
        text,
        icon,
        ...options,
      });
    };
  }
};
