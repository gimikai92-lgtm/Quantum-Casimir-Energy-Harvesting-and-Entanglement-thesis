# Zenodo Registration Checklist

Quick checklist for registering this thesis on Zenodo for permanent archival and DOI assignment.

---

## Pre-Registration Checklist

- [ ] Have GitHub account access (for linking)
- [ ] Have institutional email (recommended)
- [ ] Read [ZENODO_REGISTRATION.md](ZENODO_REGISTRATION.md)
- [ ] Download [Quantum-Casimir-Thesis-v1.0-final.tar.gz](Quantum-Casimir-Thesis-v1.0-final.tar.gz)
- [ ] Verify SHA-256: `7bd9f060a902eabc18f12400c5748c7a4d0e4e3a2fcd7bc1cf7c90945a5a4c4d`

---

## Step-by-Step Registration

### 1. Account Setup (5 minutes)
- [ ] Visit https://zenodo.org/signup
- [ ] Create account with email
- [ ] Verify email
- [ ] Complete profile:
  - [ ] Full name: **Rolando M Garcia**
  - [ ] Institution: **[Your Institution Name]**
  - [ ] ORCID: [If available]

### 2. File Preparation (2 minutes)
- [ ] Archive file ready: `Quantum-Casimir-Thesis-v1.0-final.tar.gz` (92 KB)
- [ ] Checksum verified: `7bd9f060a902eabc18f12400c5748c7a4d0e4e3a2fcd7bc1cf7c90945a5a4c4d`
- [ ] File size acceptable: ✓ (< 50 GB limit)

### 3. Upload (3 minutes)
- [ ] Log into https://zenodo.org
- [ ] Click "Upload"
- [ ] Select "New Upload"
- [ ] Click "Publish Later"
- [ ] Drag/drop or select: `Quantum-Casimir-Thesis-v1.0-final.tar.gz`
- [ ] Wait for upload confirmation

### 4. Metadata Entry (10 minutes)

#### Required Fields
- [ ] **Publication Date:** `2026-02-01`
- [ ] **Title:** `Quantum-Casimir Energy Harvesting and Entanglement-Directed Energy Management in Lithium-Nitrogen Co-Doped Graphene Quantum Systems`
- [ ] **Author:** `Rolando M Garcia`
- [ ] **Description:** [Copy abstract from Chapter 0]
- [ ] **Upload Type:** Select `Doctoral Thesis`
- [ ] **License:** Select `CC BY-NC-ND 4.0 International`

#### Recommended Fields
- [ ] **Keywords:** `quantum-physics, casimir-effect, quantum-entanglement, energy-harvesting, graphene, thesis, research`
- [ ] **Subject:** Physics
- [ ] **Discipline:** Quantum Physics
- [ ] **Institution:** [Your Institution]
- [ ] **Related Identifiers:**
  - Type: `URL`
  - Value: `https://github.com/gimikai92-lgtm/Quantum-Casimir-Energy-Harvesting-and-Entanglement-thesis`
  - Relation: `isSupplementTo`
- [ ] **Notes:** `Patent-pending. See LICENSE for usage restrictions.`

#### Creator Information
- [ ] **Name:** Rolando M Garcia
- [ ] **Affiliation:** [Your Institution]
- [ ] **ORCID:** [If available]

### 5. Review & Publish (3 minutes)
- [ ] Review all metadata
- [ ] Verify license: CC BY-NC-ND 4.0
- [ ] Confirm author name
- [ ] Click "Publish"
- [ ] **Wait for DOI assignment** (Usually instant)

### 6. Record DOI (1 minute)
Once published, you'll see:
```
DOI: 10.5281/zenodo.XXXXXXX
```

Copy this value: `_________________________`

---

## Post-Registration Tasks

### Update Thesis Files

**1. Update CITATION.cff:**
```bash
nano CITATION.cff
# Add to identifiers section:
identifiers:
  - type: doi
    value: 10.5281/zenodo.XXXXXXX
```
- [ ] Add DOI to CITATION.cff
- [ ] Save file

**2. Update README.md:**
Add near top of file:
```markdown
**DOI:** [10.5281/zenodo.XXXXXXX](https://doi.org/10.5281/zenodo.XXXXXXX)
**Zenodo Archive:** Permanent digital preservation with 50+ year guarantee
```
- [ ] Add DOI badge to README
- [ ] Add Zenodo link

**3. Update DATA_DICTIONARY.md:**
```markdown
### Online Repositories
- **Zenodo:** https://doi.org/10.5281/zenodo.XXXXXXX
```
- [ ] Update Zenodo link in data dictionary

**4. Commit changes:**
```bash
git add CITATION.cff README.md DATA_DICTIONARY.md
git commit -m "Register thesis on Zenodo - DOI: 10.5281/zenodo.XXXXXXX"
git push origin main
```
- [ ] Commit with DOI information
- [ ] Push to GitHub

---

## Verification Checklist

### Zenodo Verification
- [ ] Record visible at: https://zenodo.org/records/XXXXXXX
- [ ] DOI resolves: https://doi.org/10.5281/zenodo.XXXXXXX
- [ ] Metadata complete and accurate
- [ ] License correctly displayed
- [ ] File downloadable
- [ ] Citation available

### GitHub Verification
- [ ] Latest commit includes DOI
- [ ] CITATION.cff updated
- [ ] README reflects Zenodo registration
- [ ] All files synced

### Citation Verification
- [ ] BibTeX citation works
- [ ] DOI in citation format
- [ ] Can be found via Google Scholar
- [ ] Citation count tracking enabled

---

## Zenodo DOI Information

| Element | Value |
|---------|-------|
| **DOI Format** | 10.5281/zenodo.XXXXXXX |
| **Full URL** | https://doi.org/10.5281/zenodo.XXXXXXX |
| **Persistent?** | Yes (50+ years guaranteed) |
| **Citable?** | Yes (automatically indexed) |
| **Cost** | Free |
| **Restrictions** | None (open access) |

---

## Citation Examples

Once you have your DOI, use these formats:

### BibTeX
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

### APA Format
Garcia, R. M. (2026). Quantum-Casimir energy harvesting and entanglement-directed energy management in lithium-nitrogen co-doped graphene quantum systems [Doctoral thesis]. https://doi.org/10.5281/zenodo.XXXXXXX

### MLA Format
Garcia, Rolando M. Quantum-Casimir Energy Harvesting and Entanglement-Directed Energy Management in Lithium-Nitrogen Co-Doped Graphene Quantum Systems. 2026. https://doi.org/10.5281/zenodo.XXXXXXX

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| **DOI not appearing** | Wait 5-10 min, refresh page, check email |
| **Upload failed** | Check file size, try different format, clear cache |
| **Metadata won't save** | Edit before publishing, may need new version |
| **License not showing** | Ensure CC BY-NC-ND 4.0 is selected |
| **Need to update** | Create new version with incremented DOI |

**Support:** zenodo-support@cern.ch

---

## Timeline & Status

- [ ] **Step 1-2:** Account & File Prep (5-10 min)
- [ ] **Step 3-4:** Upload & Metadata (15 min)
- [ ] **Step 5-6:** Publish & Record DOI (5 min)
- [ ] **Post-Reg:** Update files & commit (10 min)

**Total Time:** ~45 minutes to complete registration

---

## Recommended Next Actions

1. **Complete Zenodo registration** (follow steps above)
2. **Share DOI** with:
   - [ ] Institution/department
   - [ ] Dissertation office
   - [ ] Advisor/committee
   - [ ] Institutional repository
3. **Submit to databases:**
   - [ ] Google Scholar (automatic via Zenodo)
   - [ ] arXiv (if preprint)
   - [ ] Institutional repository
4. **Announce publication:**
   - [ ] LinkedIn (career announcement)
   - [ ] ResearchGate (profile update)
   - [ ] Twitter/social media
5. **Monitor metrics:**
   - [ ] Download statistics
   - [ ] Citation tracking
   - [ ] Altmetrics

---

## Important Notes

- ✓ Registration is **free** (CERN funded)
- ✓ DOI is **permanent** (50+ year preservation)
- ✓ Access is **open** (matches CC BY-NC-ND 4.0)
- ✓ Indexed by **Google Scholar** automatically
- ✓ Version control possible (new DOIs for updates)

---

## Additional Resources

- **Zenodo Help:** https://about.zenodo.org/help/
- **Zenodo FAQ:** https://zenodo.org/faq
- **DOI Guide:** https://www.doi.org/
- **Full Guide:** [ZENODO_REGISTRATION.md](ZENODO_REGISTRATION.md)

---

**Status:** Ready to register  
**Estimated Duration:** 45 minutes  
**Benefit:** Permanent digital archival + DOI + indexing  

**Start registration now:** https://zenodo.org/signup

---

*Save this checklist. You'll need your DOI value in the blank above (10.5281/zenodo.XXXXXXX).*
