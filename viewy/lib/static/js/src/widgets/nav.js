function normalizePathname(pathname) {
  if (!pathname || pathname === "/") {
    return "/";
  }

  return pathname.replace(/\/+$/, "") || "/";
}

function getNavRoots(root) {
  const navs = Array.from(root.querySelectorAll(".nav"));

  if (typeof root.matches === "function" && root.matches(".nav")) {
    navs.unshift(root);
  }

  return navs;
}

function isActiveLink(link, currentUrl) {
  const href = link.getAttribute("href");
  if (!href) {
    return false;
  }

  let targetUrl;
  try {
    targetUrl = new URL(href, currentUrl.href);
  } catch {
    return false;
  }

  if (targetUrl.origin !== currentUrl.origin) {
    return false;
  }

  const targetPath = normalizePathname(targetUrl.pathname);
  const currentPath = normalizePathname(currentUrl.pathname);

  const pathMatches =
    targetPath === "/"
      ? currentPath === "/"
      : currentPath === targetPath || currentPath.startsWith(targetPath + "/");

  if (!pathMatches) {
    return false;
  }

  if (targetUrl.search && targetUrl.search !== currentUrl.search) {
    return false;
  }

  if (targetUrl.hash && targetUrl.hash !== currentUrl.hash) {
    return false;
  }

  return true;
}

export function init(root) {
  const currentUrl = new URL(window.location.href);

  for (const nav of getNavRoots(root)) {
    for (const item of nav.querySelectorAll(".nav-item")) {
      item.classList.remove("nav-item--active");
    }

    for (const link of nav.querySelectorAll(".nav-item a[href]")) {
      const item = link.closest(".nav-item");
      const isActive = isActiveLink(link, currentUrl);

      if (isActive) {
        link.setAttribute("aria-current", "page");
        item?.classList.add("nav-item--active");
      } else {
        link.removeAttribute("aria-current");
        item?.classList.remove("nav-item--active");
      }
    }
  }
}
