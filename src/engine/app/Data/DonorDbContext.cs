using app.Models;
using Microsoft.EntityFrameworkCore;

namespace app.Data;

public class DonorDbContext(DbContextOptions<DonorDbContext> options) : DbContext(options)
{
    public DbSet<DonorProfile> DonorProfiles { get; set; }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder
            .Entity<DonorProfile>()
            .Property(d => d.Id)
            .HasDefaultValueSql("gen_random_uuid()");

        modelBuilder.Entity<DonorProfile>().Property(d => d.Email).IsRequired().HasMaxLength(255);
    }
}
