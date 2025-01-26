using System;
using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace app.Migrations
{
    /// <inheritdoc />
    public partial class FixPrimaryKey : Migration
    {
        /// <inheritdoc />
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.CreateTable(
                name: "DonorProfiles",
                columns: table => new
                {
                    Id = table.Column<Guid>(type: "uuid", nullable: false, defaultValueSql: "gen_random_uuid()"),
                    Nome = table.Column<string>(type: "text", nullable: true),
                    Email = table.Column<string>(type: "character varying(255)", maxLength: 255, nullable: false),
                    Horario = table.Column<string>(type: "text", nullable: true),
                    Doador = table.Column<bool>(type: "boolean", nullable: false),
                    LocalDeCobranca = table.Column<string>(type: "text", nullable: true),
                    Morada = table.Column<string>(type: "text", nullable: true),
                    Freguesia = table.Column<string>(type: "text", nullable: true),
                    Concelho = table.Column<string>(type: "text", nullable: true),
                    CodigoPostal = table.Column<string>(type: "text", nullable: true),
                    TelResidencial = table.Column<string>(type: "character varying(9)", maxLength: 9, nullable: true),
                    TelTrabalho = table.Column<string>(type: "character varying(9)", maxLength: 9, nullable: true),
                    Telemovel = table.Column<string>(type: "character varying(9)", maxLength: 9, nullable: false),
                    Observacoes = table.Column<string>(type: "text", nullable: true)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_DonorProfiles", x => x.Id);
                });
        }

        /// <inheritdoc />
        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropTable(
                name: "DonorProfiles");
        }
    }
}
