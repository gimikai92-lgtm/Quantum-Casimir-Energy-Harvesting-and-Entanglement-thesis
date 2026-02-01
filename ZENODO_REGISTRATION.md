# Zenodo Registration Guide

This guide explains how to register this thesis on Zenodo for permanent archival and DOI assignment.

---

## What is Zenodo?

[Zenodo](https://zenodo.org) is a free, open-access research repository managed by CERN that provides:
- ✓ Permanent digital preservation
- ✓ Digital Object Identifier (DOI) assignment
- ✓ Long-term accessibility (50+ years guaranteed)
- ✓ Citation tracking and metrics
- ✓ Community visibility and discoverability
- ✓ License flexibility (CC BY-NC-ND 4.0 supported)

---

## Steps to Register on Zenodo

### 1. Create a Zenodo Account

- Visit [https://zenodo.org/signup](https://zenodo.org/signup)
- Create account using:
  - Email address (use institutional email recommended)
  - Strong password
  - Verify email
- Complete profile setup:
  - Full name: Rolando M Garcia
  - Institution: [Your Institution]
  - ORCID (if available)

### 2. Prepare Thesis Files

**Archive Package:**
```bash
# Use existing package
Quantum-Casimir-Thesis-v1.0-final.tar.gz (92 KB)

# Or create new one with all files
tar -czf quantum-casimir-thesis-v1.0.tar.gz .
```

**File Format:**
- ✓ .tar.gz (recommended)
- ✓ .pdf (if compiled)
- ✓ .zip (alternative)
- ✓ Individual markdown files

**File Size Limit:** 50 GB (more than sufficient)

### 3. Upload to Zenodo

1. Log into [https://zenodo.org](https://zenodo.org)
2. Click **"Upload"** button
3. Select **"New Upload"**
4. Choose **"Publish Later"** (to add metadata first)
5. Upload file:
   - Drag and drop or click to select
   - Wait for upload completion
   - Confirm file integrity

### 4. Fill in Metadata

**Required Fields:**

| Field | Value |
|-------|-------|
| **Publication Date** | 2026-02-01 |
| **Title** | Quantum-Casimir Energy Harvesting and Entanglement-Directed Energy Management in Lithium-Nitrogen Co-Doped Graphene Quantum Systems |
| **Author(s)** | Rolando M Garcia |
| **Description** | [Use thesis abstract from Chapter 0] |
| **Type** | Doctoral Thesis |
| **License** | CC BY-NC-ND 4.0 International |

**Recommended Fields:**

| Field | Value |
|-------|-------|
| **Keywords** | quantum-physics, casimir-effect, quantum-entanglement, energy-harvesting, graphene |
| **Subject** | Physics |
| **Discipline** | Quantum Physics |
| **Institution** | [Your Institution] |
| **Related Identifiers** | GitHub URL, arXiv (if applicable) |
| **Notes** | Patent-pending. See LICENSE for usage terms. |

**Creator Information:**
- Name: Rolando M Garcia
- Affiliation: [Institution Name]
- ORCID: [If available]

### 5. Select License

1. Scroll to "License" section
2. Choose: **Creative Commons Attribution Non-Commercial No Derivatives 4.0 International (CC BY-NC-ND 4.0)**
3. This matches the thesis licensing model

### 6. Add Related Content

**Link Related Items:**
- GitHub Repository: `https://github.com/gimikai92-lgtm/Quantum-Casimir-Energy-Harvesting-and-Entanglement-thesis`
- Experimental Data: (Link to separate data submission if applicable)
- Code Repository: (If in separate repo)

**Relations:**
- `isVersionOf`: (If updating existing record)
- `isReferencedBy`: (Citation tracking)

### 7. Review & Publish

1. **Review** all metadata for accuracy
2. **Verify** license selection
3. **Confirm** author information
4. **Click** "Publish"
5. **Wait** for DOI assignment (immediate)

---

## After Publication

### Zenodo DOI

Once published, Zenodo will provide:
```
DOI: 10.5281/zenodo.XXXXXXX
```

**Update thesis files with DOI:**

1. Update [CITATION.cff](CITATION.cff):
```yaml
identifiers:
  - type: doi
    value: 10.5281/zenodo.XXXXXXX
```

2. Update [README.md](README.md):
```markdown
**DOI:** [10.5281/zenodo.XXXXXXX](https://doi.org/10.5281/zenodo.XXXXXXX)
```

3. Update [DATA_DICTIONARY.md](DATA_DICTIONARY.md):
```markdown
**Zenodo:** https://doi.org/10.5281/zenodo.XXXXXXX
```

4. Commit changes to GitHub

### Citation Format

**BibTeX with DOI:**
```bibtex
@phdthesis{Garcia2026,
  author = {Rolando M Garcia},
  title = {Quantum-Casimir Energy Harvesting and Entanglement-Directed 
           Energy Management in Lithium-Nitrogen Co-Doped Graphene 
           Quantum Systems},
  school = {[Institution Name]},
  year = {2026},
  doi = {10.5281/zenodo.XXXXXXX}
}
```

**Citation Metrics:**
- Zenodo tracks downloads and citations
- Google Scholar will index the DOI
- Impact tracking and altmetrics available

---

## Publishing Experimental Data

### Separate Data Submission (Recommended)

For large experimental datasets, consider separate Zenodo record:

**File:** `data/casimir_measurements.csv` + others

**Create New Upload:**
1. Log into Zenodo
2. Upload data files separately
3. Title: "Quantum-Casimir Energy Harvesting - Experimental Data"
4. Link to thesis DOI (as `isPartOf` relation)

**Benefits:**
- Data separately citable
- Easier version control
- Better discoverability
- Separate metrics tracking

---

## Publishing Code

### GitHub + Zenodo Integration

Zenodo can automatically archive GitHub releases:

**Steps:**

1. **Create Release on GitHub:**
   ```bash
   git tag v1.0-final
   git push origin v1.0-final
   ```

2. **Link GitHub to Zenodo:**
   - Go to [Zenodo GitHub Settings](https://zenodo.org/account/settings/github/)
   - Authorize GitHub account
   - Select repository to sync
   - Enable automatic archiving

3. **Create Release:**
   - Go to GitHub repository
   - Click "Releases"
   - "Create a new release"
   - Tag: `v1.0-final`
   - Zenodo automatically archives

**Result:** Separate DOI for each GitHub release

---

## Community Engagement

### Zenodo Features

**After Publication:**
- Add to collections (e.g., Quantum Physics)
- Enable social media sharing buttons
- Track downloads and citations
- Respond to user comments
- Update record with newer versions

**Metrics Available:**
- Total downloads
- Geographic distribution
- Citation tracking
- Altmetrics scores

### Linking Records

**Connect Related Works:**
- Link data to thesis
- Link code to thesis
- Link presentations to thesis
- Create comprehensive research footprint

---

## Version Management

### Updates & Versions

If updating thesis after initial publication:

1. **Minor Updates** (typo fixes):
   - Edit Zenodo record directly
   - Increment version in filename
   - Add change note

2. **Major Updates** (significant revisions):
   - Create new Zenodo record
   - Link as `isNewVersionOf` to original
   - Assign new DOI
   - Reference original in citations

### Versioning Scheme

```
Original:  10.5281/zenodo.XXXXXXX (v1.0)
Update:    10.5281/zenodo.YYYYYYY (v1.1)
Revision:  10.5281/zenodo.ZZZZZZZ (v2.0)
```

All versions remain accessible and citable.

---

## Compliance & Access

### Open Access Statement

Include this statement in your thesis:

> "This thesis is available open access via Zenodo under the Creative Commons Attribution Non-Commercial No Derivatives 4.0 International License. For full terms and conditions, see [LICENSE](LICENSE)."

### Access Control

**Zenodo Access Options:**
- ✓ Open Access (current recommendation)
- ✓ Embargoed (delayed access)
- ✓ Restricted (requires approval)
- ✓ Closed (requires Zenodo login)

**Recommended:** Open Access (matches CC BY-NC-ND 4.0)

---

## Long-term Preservation

### Zenodo Guarantees

- **Preservation Period:** Minimum 50 years
- **Format Migration:** CERN will migrate formats as needed
- **Physical Distribution:** Multiple geographically distributed copies
- **Access:** Persistent DOI remains functional
- **Cost:** Free (CERN funded)

### Your Responsibilities

- Provide accurate metadata
- Maintain valid contact information
- Update when significant changes occur
- Report any errors or corrections

---

## Institutional Repositories

### Complementary Archival

Consider also depositing in:
- **Institutional Repository:** [Your University]
- **arXiv (if Preprint):** https://arxiv.org
- **SSRN (Economics):** https://ssrn.com
- **ResearchGate:** https://researchgate.net

**Benefits:**
- Multiple access points
- Institutional discoverability
- Discipline-specific indexing
- Backup archival

---

## Troubleshooting

### Common Issues

**DOI Not Appearing?**
- Wait 5-10 minutes after publication
- Refresh page
- Check "Publish" was clicked
- Contact Zenodo support

**Metadata Not Updating?**
- Edit the record before publishing
- Can edit after publication in limited ways
- May need to create new version

**File Upload Failed?**
- Check file size (< 50 GB)
- Try different format
- Check browser compatibility
- Contact Zenodo support

---

## Support & Contact

### Zenodo Help
- **Help:** https://about.zenodo.org/help/
- **FAQ:** https://zenodo.org/faq
- **Support:** zenodo-support@cern.ch

### Additional Resources
- [Zenodo User Guide](https://zenodo.org/features)
- [Scholarly Kitchen on Zenodo](https://scholarlykitchen.sspnet.org/)
- [DOI Best Practices](https://www.doi.org/)

---

## Example DOI Citation

Once registered, your DOI citation will look like:

```
Zenodo DOI: 10.5281/zenodo.1234567
Full URL: https://doi.org/10.5281/zenodo.1234567
```

Share this DOI in:
- CV and publication lists
- Social media announcements
- Institutional repositories
- Research databases

---

## Next Steps

1. ✓ Prepare thesis files
2. → Create Zenodo account
3. → Upload thesis package
4. → Fill metadata
5. → Publish and receive DOI
6. → Update GitHub and documentation
7. → Share DOI with institution
8. → Announce publication

---

**Status:** Ready to implement  
**Timeline:** < 30 minutes to complete  
**Cost:** Free  
**Impact:** High (permanent archival + DOI)

For assistance, see [Zenodo documentation](https://zenodo.org) or contact institutional librarian.
