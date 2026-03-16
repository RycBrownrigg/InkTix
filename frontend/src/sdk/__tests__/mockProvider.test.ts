import { describe, it, expect, beforeEach } from "vitest";
import { MockProvider } from "../mockProvider";

describe("MockProvider", () => {
  let provider: MockProvider;

  beforeEach(() => {
    provider = new MockProvider();
  });

  describe("Team Management", () => {
    it("should register a team and return an id", async () => {
      const result = await provider.registerTeam(
        "Celtics",
        "Boston",
        "Basketball"
      );
      expect(result.success).toBe(true);
      expect(result.data).toBeGreaterThan(0);
    });

    it("should get a team by id", async () => {
      const result = await provider.getTeam(1);
      expect(result.success).toBe(true);
      expect(result.data?.name).toBe("Lakers");
    });

    it("should return error for non-existent team", async () => {
      const result = await provider.getTeam(999);
      expect(result.success).toBe(false);
    });

    it("should return all teams", async () => {
      const result = await provider.getAllTeams();
      expect(result.success).toBe(true);
      expect(result.data!.length).toBeGreaterThanOrEqual(2);
    });
  });

  describe("Artist Management", () => {
    it("should register an artist", async () => {
      const result = await provider.registerArtist("Beyonce");
      expect(result.success).toBe(true);
      expect(result.data).toBeGreaterThan(0);
    });

    it("should verify an artist", async () => {
      const result = await provider.verifyArtist(1);
      expect(result.success).toBe(true);
    });

    it("should return error for verifying non-existent artist", async () => {
      const result = await provider.verifyArtist(999);
      expect(result.success).toBe(false);
    });
  });

  describe("Venue Management", () => {
    it("should register a venue", async () => {
      const result = await provider.registerVenue(
        "Madison Square Garden",
        20000,
        "New York, NY"
      );
      expect(result.success).toBe(true);
      expect(result.data).toBeGreaterThan(0);
    });

    it("should get a venue by id", async () => {
      const result = await provider.getVenue(1);
      expect(result.success).toBe(true);
      expect(result.data?.name).toBe("Crypto.com Arena");
      expect(result.data?.capacity).toBe(19068);
    });
  });

  describe("Event Management", () => {
    it("should create a sports event", async () => {
      const result = await provider.createSportsEvent(
        "Test Game",
        1,
        Date.now(),
        19000,
        "100 DOT",
        1,
        2,
        1,
        "RegularSeason",
        "Basketball"
      );
      expect(result.success).toBe(true);
      expect(result.data).toBeGreaterThan(0);
    });

    it("should create a concert event", async () => {
      const result = await provider.createConcertEvent(
        "Test Concert",
        1,
        1,
        Date.now(),
        10000,
        "50 DOT"
      );
      expect(result.success).toBe(true);
      expect(result.data).toBeGreaterThan(0);
    });

    it("should get an event by id", async () => {
      const result = await provider.getEvent(1);
      expect(result.success).toBe(true);
      expect(result.data?.name).toBe("Lakers vs Warriors");
    });
  });

  describe("Ticket Management", () => {
    it("should purchase a ticket", async () => {
      const result = await provider.purchaseTicket(1, 201, "B", "10");
      expect(result.success).toBe(true);
      expect(result.data).toBeGreaterThan(0);
    });

    it("should get user tickets", async () => {
      const result = await provider.getUserTickets(
        "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
      );
      expect(result.success).toBe(true);
      expect(result.data!.length).toBeGreaterThanOrEqual(2);
    });

    it("should transfer a ticket", async () => {
      const result = await provider.transferTicket(1, "5FakeAddress");
      expect(result.success).toBe(true);
    });
  });

  describe("Platform Stats", () => {
    it("should return platform stats with correct shape", async () => {
      const result = await provider.getPlatformStats();
      expect(result.success).toBe(true);
      expect(result.data).toHaveProperty("totalEvents");
      expect(result.data).toHaveProperty("totalTicketsSold");
      expect(result.data).toHaveProperty("totalRevenue");
      expect(result.data).toHaveProperty("totalUsers");
      expect(result.data).toHaveProperty("averageTicketPrice");
    });
  });

  describe("Anti-Scalping", () => {
    it("should return anti-scalping config", async () => {
      const result = await provider.getAntiScalpingConfig(1);
      expect(result.success).toBe(true);
      expect(result.data?.maxTicketsPerUser).toBe(4);
      expect(result.data?.enabled).toBe(true);
    });
  });

  describe("Utility", () => {
    it("should return owner address", async () => {
      const result = await provider.getOwner();
      expect(result.success).toBe(true);
      expect(result.data).toBeTruthy();
    });

    it("should return totals with correct shape", async () => {
      const result = await provider.getTotals();
      expect(result.success).toBe(true);
      expect(result.data).toHaveProperty("teams");
      expect(result.data).toHaveProperty("venues");
      expect(result.data).toHaveProperty("events");
      expect(result.data).toHaveProperty("tickets");
    });
  });
});
