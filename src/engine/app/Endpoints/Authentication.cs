namespace app;

public static class Authentication
{
    public static void AddDonorEndpoints(this IEndpointRouteBuilder app)
    {
        var donors = app.MapGroup("/donor");

        donors.MapGet("/test/", (HttpContext ctx) =>
        {
            return Results.Ok("");
        })
        .WithName("test")
        .WithSummary("summary")
        .WithDescription("description");
    }
}
