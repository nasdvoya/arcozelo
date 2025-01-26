using app.Data;
using app.Models;
using Microsoft.AspNetCore.Mvc;

namespace app.Endpoints;

public static class Donors
{
    public static void AddDonorEndpoints(this IEndpointRouteBuilder app)
    {
        app.MapPost(
                "/donor-profile/",
                async ([FromForm] DonorProfile profile, DonorDbContext db) =>
                {
                    db.DonorProfiles.Add(profile);
                    await db.SaveChangesAsync();
                    return Results.Created($"/donor-profile/{profile.Id}", profile);
                }
            )
            .WithName("test")
            .WithSummary("summary")
            .DisableAntiforgery()
            .WithDescription("description");

        app.MapGet(
            "/donor-profile/{id}",
            async (Guid id, DonorDbContext db) =>
            {
                var donor = await db.DonorProfiles.FindAsync(id);
                return donor is not null ? Results.Ok(donor) : Results.NotFound();
            }
        );
    }
}
