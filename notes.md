1. Preparation & Consent

    Generate a unique Screening ID.
    Record screening start time, end time, timezone, examiner, and tool version.
    Obtain and document explicit, informed consent before starting.
    Explain the purpose, scope, and categories of data being inspected.
    Provide the player with an overview of the screening process.
    Define whether Standard Screening Mode or Extended Forensic Mode is being used.
    Ensure that the player can terminate the screening.
    Do not automatically collect passwords, authentication tokens, cookies, private messages, or unrelated personal data.
    Verify that the screening tool is signed, versioned, and independently auditable.
    Verify the integrity of the screening tool before execution.
    Document all permissions and access requirements.
    Ensure that the tool does not execute, modify, or delete suspicious files.

2. System Baseline

Collect only system information relevant to the screening.

    Windows version, build, and architecture.
    Active user account relevant to Minecraft.
    System timezone and current system time.
    UAC status.
    Active antivirus, anti-cheat, and endpoint security products.
    Relevant installed applications.
    Relevant running services and drivers.
    Recent Windows updates.
    VPN, proxy, and virtual network adapter configurations, where relevant.
    Available Windows Event Log sources.
    Existing Sysmon installation and configuration, if present.

Additional observations:

    Identify unexpected system-time inconsistencies.
    Record disabled or unavailable security features.
    Document potentially relevant security configuration changes.
    Distinguish legitimate system configurations from suspicious modifications.

3. Process Forensics

Collect relevant process metadata, prioritizing Minecraft and Java-related processes.

For relevant processes, record:

    PID and PPID.
    Process name.
    Executable path.
    Process start time.
    Command line, with sensitive information redacted.
    SHA-256 executable hash.
    Digital signature status and publisher.
    Parent-child process relationships.
    Associated network connections, where available.
    Relevant loaded module metadata.
    Whether the process is associated with a known launcher or legitimate application.

Special attention:

    java.exe
    javaw.exe
    Minecraft launchers.
    Overlay applications.
    Known Minecraft utility software.
    Unknown processes associated with the Minecraft runtime.

Suspicious indicators must be documented with their context and should never be treated as conclusive evidence individually.
4. Persistence & Startup Analysis

Inspect relevant persistence mechanisms using read-only analysis.

    User and system Startup folders.
    Relevant Registry autorun entries.
    Run and RunOnce entries.
    Relevant Windows services.
    Scheduled tasks.
    Relevant WMI persistence metadata.
    Startup scripts and launch configurations.
    Relevant shell extensions.
    Unusual executable paths.
    Invalid or unexpected digital signatures.

For every finding:

    Record the source location.
    Record the associated executable or script.
    Record its hash where applicable.
    Record timestamps.
    Explain why the entry is relevant.
    Distinguish legitimate software persistence from suspicious activity.

5. Minecraft-Specific Analysis

This is one of the primary screening components.
5.1 Launcher Identification

Identify installed Minecraft launchers, including:

    Official Minecraft Launcher.
    Prism Launcher.
    MultiMC.
    ATLauncher.
    CurseForge.
    Modrinth App.
    Other detected launchers.

Do not assume that all Minecraft installations are located in the default .minecraft directory.
5.2 Minecraft Instances & Versions

    Identify installed Minecraft versions.
    Identify active and recently used instances.
    Identify Vanilla, Fabric, Forge, NeoForge, and Quilt installations.
    Identify separate launcher instances and portable installations.
    Record relevant instance paths.
    Identify recently modified game configurations.
    Identify unusual or duplicate instance directories.

5.3 Mod & Library Analysis

Inspect relevant directories:

    mods
    versions
    libraries
    config
    resourcepacks
    Relevant launcher-specific directories.

For relevant files:

    Record filename and path.
    Calculate SHA-256 hashes.
    Record creation and modification timestamps.
    Check digital signatures where applicable.
    Compare against trusted reference databases.
    Identify unknown or modified Minecraft JAR files.
    Identify unexpected executable files in Minecraft directories.
    Identify suspicious configuration changes.
    Correlate installed mods with the active Minecraft instance.

5.4 Runtime Correlation

    Correlate active Java processes with their Minecraft instances.
    Compare relevant JVM arguments against expected configurations.
    Identify unexpected runtime dependencies.
    Compare loaded modules with the known Minecraft environment.
    Correlate relevant runtime findings with file-system evidence.

Unknown mods, modified JARs, and unusual JVM arguments require manual evaluation.
6. Download History Analysis

This module should focus on relevant downloads within an explicitly defined screening period.
6.1 Supported Browsers

    Google Chrome.
    Microsoft Edge.
    Brave.
    Mozilla Firefox.
    Other detected browsers, if relevant.

6.2 Relevant Download Metadata

Where access has been explicitly authorized:

    Filename.
    Download URL.
    Final download destination.
    Local file path.
    Download timestamp.
    File size.
    Download status.
    MIME type.
    Relevant browser profile.
    Whether the downloaded file still exists.

Avoid collecting unrelated browsing activity.
6.3 Download Correlation

    Correlate relevant downloads with existing files.
    Compare file hashes.
    Identify renamed or relocated files where evidence supports the relationship.
    Compare timestamps with Minecraft activity.
    Correlate relevant downloads with Minecraft directories.
    Correlate relevant downloads with known software indicators.
    Document missing or unavailable evidence.

6.4 General Download Locations

Prioritize relevant files in:

    Downloads.
    Desktop.
    Documents, where relevant.
    AppData.
    LocalAppData.
    Temp directories.
    Minecraft directories.
    Relevant launcher directories.

Prioritize executable and archive formats:

    .exe
    .dll
    .jar
    .zip
    .rar
    .7z
    .msi
    .bat
    .cmd
    .ps1
    .vbs

Inspect archives without executing their contents.
6.5 Historical Evidence

Extended Forensic Mode only:

    Consider relevant browser database remnants.
    Consider available filesystem metadata.
    Document whether historical evidence is incomplete.
    Distinguish recovered artifacts from current files.
    Never interpret deleted-file traces as proof of execution or cheating.

7. File System Analysis

Use a risk-based approach.
7.1 Priority Locations

    Minecraft directories.
    Launcher directories.
    Downloads.
    Desktop.
    Relevant AppData directories.
    Temp directories.
    Recently modified relevant files.

7.2 File Metadata

For relevant files:

    Filename.
    Full path.
    File type.
    File size.
    SHA-256 hash.
    Creation timestamp.
    Modification timestamp.
    Digital signature status.
    Associated application.
    Detection rule and reason.

7.3 File Classification

Classify files as:

    Known legitimate.
    Known suspicious.
    Unknown.
    Modified.
    Potentially malicious.
    Confirmed match to a verified cheat artifact.

Never classify software as cheating solely because it is unsigned, unknown, hidden, or located in an unusual directory.
8. Runtime & Memory Evidence

Extended Forensic Mode only.

Only investigate processes where a concrete and documented reason exists.

    Identify relevant Minecraft process activity.
    Record relevant process metadata.
    Document unusual loaded modules.
    Document relevant memory-analysis findings.
    Correlate findings with known legitimate software.
    Distinguish observed behavior from inferred behavior.
    Document limitations of the analysis.
    Obtain separate authorization before collecting sensitive memory artifacts.
    Encrypt any authorized memory captures.
    Define a retention and deletion policy.

Memory-related indicators must not automatically result in a cheating classification.
9. Network Analysis

Use limited, relevant network observations.

    Record relevant active TCP and UDP connections.
    Associate connections with processes where possible.
    Record relevant connection timestamps.
    Identify configured VPN and proxy software.
    Record relevant DNS information where authorized.
    Correlate suspicious connections with process metadata.
    Correlate relevant network observations with the screening timeline.

Do not classify a player as cheating based solely on connections to unknown IP addresses, domains, VPNs, or proxies.
10. Windows Event Logs

Inspect relevant available logs.

    Security.
    System.
    Application.
    PowerShell.
    Sysmon, if installed.

Prioritize relevant events involving:

    Process creation.
    Service installation.
    Driver installation.
    Relevant application execution.
    Security configuration changes.
    Relevant file-system activity.
    Relevant Registry activity.
    Available network telemetry.

Additional requirements:

    Record log source and event identifiers.
    Record timestamps and timezone.
    Document unavailable or cleared logs.
    Distinguish missing telemetry from evidence of deliberate deletion.
    Correlate relevant events with the screening timeline.

11. Browser Extensions & Clipboard
11.1 Browser Extensions

Where relevant and authorized:

    Identify installed browser extensions.
    Record extension names and identifiers.
    Check whether extensions are known legitimate software.
    Document unexpected or suspicious extensions.
    Evaluate findings in the context of Minecraft-related activity.

11.2 Clipboard

    Do not collect clipboard contents by default.
    Require separate, explicit authorization for any clipboard inspection.
    Never automatically collect passwords, authentication tokens, or unrelated private information.

12. Server-Side Correlation

Use independent server-side evidence to complement client screening.

    Associate the Screening ID with the relevant match ID.
    Verify the player's Minecraft UUID.
    Record the relevant gameplay time window.
    Correlate client findings with server-side anti-cheat observations.
    Include relevant replay references where available.
    Document independently observed suspicious gameplay behavior.
    Compare client and server timestamps.
    Distinguish behavioral evidence from technical evidence.

Server-side observations and client-side artifacts must remain separate evidence categories.
13. Anti-Evasion & Screening Integrity

Focus on the reliability of the screening process itself.

    Verify scanner component integrity.
    Record scanner version and configuration.
    Detect incomplete or interrupted checks.
    Record unavailable data sources.
    Document permission-related limitations.
    Identify inconsistencies between collected evidence sources.
    Record skipped checks and reasons.
    Ensure the final report accurately represents the actual screening scope.
    Never classify an incomplete screening as clean.

14. Evidence Integrity & Chain of Custody

Every relevant artifact must be traceable.

    Assign a unique Evidence ID.
    Record the original source location.
    Record acquisition timestamp.
    Record acquisition method.
    Calculate SHA-256 hashes where applicable.
    Preserve original evidence without modification.
    Separate original evidence from derived analysis.
    Record every analysis operation.
    Maintain an append-only audit log.
    Sign the final screening report.
    Ensure that report integrity can be independently verified.
    Restrict evidence access to authorized reviewers.

For every finding, clearly distinguish:

    Observed facts.
    Analytical interpretation.
    Confidence level.
    Remaining uncertainties.

15. Privacy & Data Protection

    Collect only data necessary for the stated screening purpose.
    Clearly define the screening scope before execution.
    Avoid unrelated personal files and application data.
    Never collect passwords, cookies, authentication tokens, or session credentials.
    Redact sensitive information from reports.
    Encrypt stored evidence.
    Restrict access to authorized reviewers.
    Maintain an access log.
    Define a fixed evidence-retention period.
    Securely delete evidence when retention is no longer justified.
    Document consent and withdrawal.
    Provide the player with information about the collected data.
    Review applicable data protection and legal requirements before deployment.

16. Finding Classification & Confidence Levels

Every finding must receive a classification.
Classification	Meaning
Clean	No relevant cheating indicators identified within the screening scope.
Suspicious	One or more indicators require additional context.
Likely Cheat	Multiple independent indicators strongly support a cheating hypothesis.
Confirmed Cheat	Reliable, independently verifiable evidence establishes the presence and relevant use of a cheating artifact or functionality.
Inconclusive	Available evidence is insufficient for a reliable conclusion.
Evidence Weighting
Finding	Interpretation
Unknown file	Weak indicator
Download record only	Indicator requiring context
Download record + matching file	Stronger indicator
Relevant file + verified execution evidence	Strong finding
Verified cheat artifact + relevant runtime evidence	Very strong finding
Multiple independent findings + matching gameplay evidence	Potentially decisive evidence

Additional requirements:

    A download entry alone must never result in disqualification.
    An unknown file alone must never result in disqualification.
    An unsigned file alone must never result in disqualification.
    Hash matches must be verified against trusted databases.
    Distinguish file presence from actual execution.
    Distinguish execution from use during a relevant match.
    Evaluate false-positive possibilities.
    Require independent human review for serious findings.
    Give the player an opportunity to explain relevant findings.
    Provide an appeal process.
    Never let the scanner independently issue a final penalty.

17. Reporting & Documentation

The final report should contain:
17.1 Screening Information

    Screening ID.
    Player identifier or Minecraft UUID.
    Examiner.
    Screening date and time.
    Tool version.
    Screening mode.
    Consent status.
    Screening scope.

17.2 System Information

    Relevant Windows information.
    Relevant launcher information.
    Minecraft versions and instances.
    Relevant process information.
    Relevant security software.

17.3 Technical Findings

    Process findings.
    Minecraft-specific findings.
    Mod and library findings.
    Download history findings.
    File-system findings.
    Persistence findings.
    Relevant network observations.
    Relevant Windows Event Log findings.
    Server-side correlation, where available.

17.4 Evidence Documentation

For each finding:

    Evidence ID.
    Detection rule.
    Source location.
    SHA-256 hash, where applicable.
    Timestamp.
    Observed facts.
    Analytical interpretation.
    Confidence level.
    False-positive considerations.
    Manual review status.

17.5 Final Assessment

    Overall classification.
    Relevant supporting evidence.
    Relevant contradictory evidence.
    Missing or unavailable evidence.
    Screening limitations.
    Reviewer recommendation.
    Final human decision.
    Appeal status, where applicable.

18. Testing & Quality Assurance

Before releasing the screening tool:

    Test against clean Windows installations.
    Test against clean Vanilla Minecraft installations.
    Test against legitimate Fabric and Forge installations.
    Test against popular legitimate mods.
    Test against different Minecraft launchers.
    Test against multiple Minecraft instances.
    Test report integrity and reproducibility.
    Test incomplete and interrupted screenings.
    Test permission-related failures.
    Validate detection rules against known false positives.
    Maintain versioned detection databases.
    Document detection rule changes.
    Independently review important detection rules.
    Verify that scanner updates do not silently change the screening scope.

19. Operating Modes
Standard Screening Mode

Designed for routine tournament screenings.

Includes:

    System baseline.
    Relevant process metadata.
    Minecraft launcher and instance identification.
    Minecraft mod and library analysis.
    Risk-based file analysis.
    Relevant download metadata, within the agreed scope.
    Basic persistence analysis.
    Evidence correlation.
    Standard screening report.

Characteristics:

    Minimal data collection.
    Limited system impact.
    Transparent screening scope.
    No invasive memory acquisition.
    No broad recovery of deleted data.

Extended Forensic Mode

Designed for specific, documented suspicions.

Requires separate authorization and a justified scope.

May include:

    Extended historical evidence analysis.
    Additional Windows Event Log analysis.
    More detailed process and runtime evidence.
    Authorized memory analysis.
    Additional file-system metadata analysis.
    Extended timeline reconstruction.
    Additional evidence correlation.

Characteristics:

    Explicitly documented justification.
    Separate consent.
    Strict evidence handling.
    Increased privacy safeguards.
    Independent human review.

20. Final Principles

The screening tool must follow these principles:

    Consent First: No screening without informed consent.
    Data Minimization: Only collect information relevant to the screening purpose.
    Read-Only Analysis: Never modify the player's original files.
    Evidence Over Assumptions: Unknown or suspicious does not automatically mean cheating.
    Correlation Over Isolated Indicators: Independent evidence sources should support important conclusions.
    Transparency: Every finding must have an understandable explanation.
    Reproducibility: Important findings must be independently verifiable.
    Privacy: Personal information must be protected.
    Human Review: No automatic disqualification.
    Fairness: Players must have an opportunity to respond and appeal.
    Integrity: Incomplete screenings must never be presented as clean.
    Accountability: Every screening decision must be documented.