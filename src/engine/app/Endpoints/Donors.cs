using app.Data;
using app.Models;
using Microsoft.AspNetCore.Mvc;

namespace app.Endpoints;

public static class Donors
{
    public static void AddDonorEndpoints(this IEndpointRouteBuilder app)
    {
        app.MapPost(
                "/donor/profile/",
                async ([FromBody] DonorProfile profile, DonorDbContext db, HttpResponse response) =>
                {
                    Console.WriteLine(profile.Email);
                    db.DonorProfiles.Add(profile);
                    await db.SaveChangesAsync();
                    response.Headers["HX-Redirect"] = "/index";
                    return Results.Created($"/donor/profile/{profile.Id}", profile);
                    // return Results.Json(new { success = true });
                }
            )
            .WithName("test")
            .WithSummary("summary")
            .DisableAntiforgery()
            .WithDescription("description");

        app.MapGet(
            "/donor/profile/{id}",
            async (Guid id, DonorDbContext db) =>
            {
                var donor = await db.DonorProfiles.FindAsync(id);
                return donor is not null ? Results.Ok(donor) : Results.NotFound();
            }
        );

        app.MapPost(
            "/donor/test",
            async (HttpContext context) =>
            {
                context.Response.Headers["HX-Redirect"] = "/";
                return Results.Ok();
            }
        );
    }
}
