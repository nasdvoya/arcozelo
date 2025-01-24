using Serilog;

namespace app;

public class Program
{
    public static void Main(string[] args)
    {
        Serilog.Core.Logger logger = new LoggerConfiguration()
            .MinimumLevel.Information()
            .WriteTo.File("logs/log.txt", rollingInterval: RollingInterval.Day)
            .CreateLogger();

        var builder = WebApplication.CreateBuilder(args);

        // Add services to the container.
        builder.Logging.AddSerilog(logger);
        builder.Services.AddAuthorization();
        builder.Services.AddEndpointsApiExplorer();
        builder.Services.AddSwaggerGen();
        var app = builder.Build();

        if(app.Environment.IsDevelopment())
        {
            app.UseSwagger();
            app.UseSwaggerUI();
        }

        app.UseHttpsRedirection();
        app.UseAuthorization();
        app.AddDonorEndpoints();

        app.Run();
    }
}
