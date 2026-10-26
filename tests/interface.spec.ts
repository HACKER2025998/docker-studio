import { test, expect, type Page } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

async function nativeFixture(page: Page) {
  await page.addInitScript(() => {
    const scope = window as unknown as {
      isTauri: boolean;
      __TAURI_INTERNALS__: unknown;
      __TEST_CALLS__: unknown[];
    };
    scope.isTauri = true;
    scope.__TEST_CALLS__ = [];
    scope.__TAURI_INTERNALS__ = {
      transformCallback: () => 1,
      unregisterCallback: () => undefined,
      invoke: async (
        command: string,
        args: { action?: string; data?: Record<string, unknown> },
      ) => {
        if (command.startsWith("plugin:event|")) return 1;
        scope.__TEST_CALLS__.push(args);
        switch (args.action) {
          case "doctor":
            return {
              os: "linux",
              arch: "x86_64",
              distribution: "TEST FIXTURE",
              ready: true,
              checks: [
                {
                  id: "daemon",
                  title: "Démon et accès utilisateur",
                  ok: true,
                  detail: "Fixture de test",
                },
              ],
              addresses: ["127.0.0.1", "192.168.1.45"],
              installation: "",
            };
          case "port":
            return { available: args.data?.port !== 5432, suggested: 5433 };
          case "networks":
            return [{ Name: "dockforge-dev" }];
          case "generate_password":
            return "Generated_for_test_1234!";
          case "create":
            return {
              id: "isolated-test",
              name: args.data?.name,
              readiness: "healthy",
            };
          case "vault":
            return [];
          case "containers":
            return [];
          case "commands":
            return [];
          default:
            return [];
        }
      },
    };
  });
}

test("le navigateur explore sans prétendre accéder au système", async ({
  page,
}) => {
  await page.goto("/");
  await expect(
    page.getByText("Accès système réservé au desktop"),
  ).toBeVisible();
  await page
    .getByRole("button", { name: "Container Forge", exact: false })
    .click();
  await expect(page.locator(".service-card")).toHaveCount(35);
  await page.getByRole("button", { name: "Bases SQL", exact: true }).click();
  await expect(page.locator(".service-card")).toHaveCount(5);
  await page
    .getByRole("button", { name: "Créer un conteneur PostgreSQL", exact: true })
    .click();
  await expect(page.getByRole("dialog")).toBeVisible();
  await page
    .getByLabel("POSTGRES_PASSWORD", { exact: true })
    .fill("test-password");
  await page.getByRole("button", { name: "Continuer" }).click();
  await page.getByRole("button", { name: "Continuer" }).click();
  await expect(
    page.getByRole("button", { name: "Créer & lancer" }),
  ).toBeDisabled();
  await page.keyboard.press("Escape");
  await expect(page.getByRole("dialog")).toHaveCount(0);
});

test("le parcours natif conserve port, variables et volume explicitement choisis", async ({
  page,
}) => {
  await nativeFixture(page);
  await page.goto("/");
  await expect(page.getByText("Votre atelier est prêt.")).toBeVisible();
  await page
    .getByRole("button", { name: "Container Forge", exact: false })
    .click();
  await page
    .getByRole("button", { name: "Créer un conteneur PostgreSQL", exact: true })
    .click();
  await page
    .getByRole("button", { name: "Générer POSTGRES_PASSWORD", exact: true })
    .click();
  await page.getByRole("button", { name: "Continuer" }).click();
  await page.getByRole("button", { name: "Utiliser 5433" }).click();
  await expect(
    page.getByText("Port disponible au moment du contrôle"),
  ).toBeVisible();
  await page.getByRole("button", { name: "Continuer" }).click();
  await page.getByRole("button", { name: "Créer & lancer" }).click();
  await expect(
    page.getByText("Sa sonde de santé confirme que le service répond."),
  ).toBeVisible();
  const calls = await page.evaluate(
    () =>
      (
        window as unknown as {
          __TEST_CALLS__: { action: string; data: Record<string, unknown> }[];
        }
      ).__TEST_CALLS__,
  );
  const data = calls.find((c) => c.action === "create")?.data;
  expect(data).toMatchObject({
    host_port: 5433,
    bind_ip: "127.0.0.1",
    storage: "volume",
    volume_target: "/var/lib/postgresql/data",
    env: { POSTGRES_PASSWORD: "Generated_for_test_1234!" },
  });
});

test("le catalogue et les espaces restent accessibles au clavier", async ({
  page,
}) => {
  await page.goto("/");
  await page.keyboard.press("Control+k");
  await expect(
    page.getByRole("textbox", { name: "Rechercher un service" }),
  ).toBeFocused();
  await page.keyboard.type("qdrant");
  await expect(page.locator(".service-card")).toHaveCount(1);
  await page.screenshot({
    path: "docs/screenshots/catalogue-filtre.png",
    fullPage: true,
  });
});

test("audit automatique des sept espaces et du formulaire", async ({
  page,
}) => {
  await page.goto("/");
  for (const name of [
    "Docker Doctor",
    "Container Forge",
    "Image Forge",
    "Conteneurs & accès",
    "Command Vault",
    "NetMap Studio",
    "ClassShare LAN",
  ]) {
    await page
      .getByRole("navigation")
      .getByRole("button", { name, exact: false })
      .click();
    if (name === "Container Forge")
      await expect(page.locator(".service-card").last()).toHaveCSS(
        "opacity",
        "1",
      );
    const results = await new AxeBuilder({ page })
      .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
      .analyze();
    expect(
      results.violations.map((v) => ({
        id: v.id,
        nodes: v.nodes.map((n) => n.target),
      })),
      name,
    ).toEqual([]);
  }
  await page
    .getByRole("navigation")
    .getByRole("button", { name: "Container Forge", exact: false })
    .click();
  await expect(page.locator(".service-card").last()).toHaveCSS("opacity", "1");
  await page.screenshot({
    path: "docs/screenshots/catalogue.png",
    fullPage: false,
  });
  await page
    .getByRole("button", { name: "Créer un conteneur PostgreSQL", exact: true })
    .click();
  await expect(page.locator(".forge-dialog")).toHaveCSS("opacity", "1");
  const results = await new AxeBuilder({ page })
    .withTags(["wcag2a", "wcag2aa", "wcag21aa"])
    .analyze();
  expect(
    results.violations.map((v) => ({
      id: v.id,
      nodes: v.nodes.map((n) => n.target),
    })),
  ).toEqual([]);
});
