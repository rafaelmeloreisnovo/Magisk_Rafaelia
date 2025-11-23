#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
═══════════════════════════════════════════════════════════════════════════════
RAFAELIA AUTHORSHIP AND BIBLIOGRAPHY SYSTEM
═══════════════════════════════════════════════════════════════════════════════

This module provides centralized authorship attribution, bibliographic references,
and legal compliance documentation for all RAFAELIA components.

Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
All Rights Reserved.

═══════════════════════════════════════════════════════════════════════════════
"""

from typing import Dict, List, Optional
from dataclasses import dataclass
from datetime import datetime


@dataclass
class Author:
    """Represents an author with full attribution information."""
    name: str
    affiliation: Optional[str] = None
    email: Optional[str] = None
    orcid: Optional[str] = None
    contribution: Optional[str] = None
    year: Optional[int] = None


@dataclass
class Publication:
    """Represents a scientific publication or reference."""
    title: str
    authors: List[str]
    venue: str
    year: int
    doi: Optional[str] = None
    isbn: Optional[str] = None
    pages: Optional[str] = None
    volume: Optional[str] = None
    number: Optional[str] = None
    publisher: Optional[str] = None
    url: Optional[str] = None
    contribution_type: str = "foundation"  # foundation, inspiration, methodology, etc.


@dataclass
class LegalFramework:
    """Represents a legal framework or standard."""
    name: str
    full_title: str
    jurisdiction: str
    year: int
    articles: Optional[List[str]] = None
    description: Optional[str] = None
    url: Optional[str] = None


class RafaeliaAuthorship:
    """
    Centralized authorship and bibliography management for RAFAELIA.
    
    This class maintains immutable records of:
    1. Original scientific foundations
    2. Legal and regulatory compliance
    3. Enhanced contributions by Rafael Melo Reis
    """
    
    # ═══════════════════════════════════════════════════════════════════════
    # PRIMARY SCIENTIFIC FOUNDATIONS (Immutable)
    # ═══════════════════════════════════════════════════════════════════════
    
    PRIMARY_AUTHORS = [
        Author(
            name="Ivan V. Oseledets",
            affiliation="Institute of Numerical Mathematics, Russian Academy of Sciences",
            contribution="Tensor-Train Decomposition - fundamental TT format",
            year=2011
        ),
        Author(
            name="Eugene E. Tyrtyshnikov",
            affiliation="Institute of Numerical Mathematics, Russian Academy of Sciences",
            contribution="Cross approximation algorithms, maxvol principle",
            year=2010
        ),
        Author(
            name="Gene H. Golub",
            affiliation="Stanford University",
            contribution="Matrix computations, SVD algorithms",
            year=2013
        ),
        Author(
            name="Charles F. Van Loan",
            affiliation="Cornell University",
            contribution="Matrix computations, numerical linear algebra",
            year=2013
        ),
        Author(
            name="Tamara G. Kolda",
            affiliation="Sandia National Laboratories",
            contribution="Tensor decompositions taxonomy",
            year=2009
        ),
        Author(
            name="Brett W. Bader",
            affiliation="Sandia National Laboratories",
            contribution="Tensor decompositions applications",
            year=2009
        ),
        Author(
            name="Álvaro Gonzalez",
            affiliation="Universidad de los Andes",
            contribution="Fibonacci sphere algorithm, golden ratio sampling",
            year=2010
        ),
    ]
    
    PRIMARY_PUBLICATIONS = [
        Publication(
            title="Tensor-Train Decomposition",
            authors=["Ivan V. Oseledets"],
            venue="SIAM Journal on Scientific Computing",
            year=2011,
            volume="33",
            number="5",
            pages="2295-2317",
            doi="10.1137/090752286",
            contribution_type="foundation"
        ),
        Publication(
            title="Incomplete Cross Approximation in the Mosaic-Skeleton Method",
            authors=["Eugene E. Tyrtyshnikov"],
            venue="Computing",
            year=2010,
            volume="64",
            pages="367-380",
            contribution_type="foundation"
        ),
        Publication(
            title="Matrix Computations",
            authors=["Gene H. Golub", "Charles F. Van Loan"],
            venue="Johns Hopkins University Press",
            year=2013,
            isbn="978-1421407944",
            publisher="Johns Hopkins University Press",
            contribution_type="foundation"
        ),
        Publication(
            title="Tensor Decompositions and Applications",
            authors=["Tamara G. Kolda", "Brett W. Bader"],
            venue="SIAM Review",
            year=2009,
            volume="51",
            number="3",
            pages="455-500",
            doi="10.1137/07070111X",
            contribution_type="foundation"
        ),
        Publication(
            title="Measurement of Areas on a Sphere Using Fibonacci and Latitude-Longitude Lattices",
            authors=["Álvaro Gonzalez"],
            venue="Mathematical Geosciences",
            year=2010,
            volume="42",
            pages="49-64",
            contribution_type="methodology"
        ),
    ]
    
    # ═══════════════════════════════════════════════════════════════════════
    # LEGAL AND REGULATORY FRAMEWORKS (Immutable)
    # ═══════════════════════════════════════════════════════════════════════
    
    LEGAL_FRAMEWORKS = [
        # International Copyright
        LegalFramework(
            name="Berne Convention",
            full_title="Berne Convention for the Protection of Literary and Artistic Works",
            jurisdiction="International",
            year=1886,
            articles=["Article 2", "Article 5", "Article 6bis"],
            description="International agreement governing copyright",
            url="https://www.wipo.int/treaties/en/ip/berne/"
        ),
        LegalFramework(
            name="WIPO Copyright Treaty",
            full_title="WIPO Copyright Treaty",
            jurisdiction="International",
            year=1996,
            articles=["Articles 4", "5", "11", "12"],
            description="Internet-age copyright treaty",
            url="https://www.wipo.int/treaties/en/ip/wct/"
        ),
        LegalFramework(
            name="TRIPS",
            full_title="Agreement on Trade-Related Aspects of Intellectual Property Rights",
            jurisdiction="International (WTO)",
            year=1994,
            articles=["Articles 9-14", "27", "39"],
            description="Intellectual property rights in international trade",
            url="https://www.wto.org/english/docs_e/legal_e/27-trips.pdf"
        ),
        
        # AI Ethics and Governance
        LegalFramework(
            name="UNESCO AI Ethics",
            full_title="UNESCO Recommendation on the Ethics of Artificial Intelligence",
            jurisdiction="International",
            year=2021,
            description="Principles: Proportionality, Do No Harm, Fairness, Transparency",
            url="https://www.unesco.org/en/artificial-intelligence/recommendation-ethics"
        ),
        LegalFramework(
            name="OECD AI Principles",
            full_title="OECD Principles on Artificial Intelligence",
            jurisdiction="International (OECD)",
            year=2019,
            description="Inclusive growth, human-centered values, transparency",
            url="https://oecd.ai/en/ai-principles"
        ),
        
        # Data Protection
        LegalFramework(
            name="GDPR",
            full_title="General Data Protection Regulation",
            jurisdiction="European Union",
            year=2016,
            articles=["Article 5", "25", "32"],
            description="Privacy by design and default, data protection",
            url="https://gdpr-info.eu/"
        ),
        LegalFramework(
            name="LGPD",
            full_title="Lei Geral de Proteção de Dados",
            jurisdiction="Brazil",
            year=2018,
            articles=["Articles 6", "46", "49"],
            description="Brazilian data protection law",
            url="https://www.planalto.gov.br/ccivil_03/_ato2015-2018/2018/lei/l13709.htm"
        ),
        
        # Quality and Security Standards
        LegalFramework(
            name="ISO 9001",
            full_title="ISO 9001:2015 Quality Management Systems",
            jurisdiction="International (ISO)",
            year=2015,
            description="Quality management principles and requirements"
        ),
        LegalFramework(
            name="ISO 27001",
            full_title="ISO/IEC 27001:2022 Information Security Management",
            jurisdiction="International (ISO/IEC)",
            year=2022,
            description="Information security management systems"
        ),
        LegalFramework(
            name="ISO 25010",
            full_title="ISO/IEC 25010:2011 Software Quality Model (SQuaRE)",
            jurisdiction="International (ISO/IEC)",
            year=2011,
            description="Software product quality model"
        ),
        
        # Software Engineering Standards
        LegalFramework(
            name="IEEE 830",
            full_title="IEEE 830-1998 Software Requirements Specification",
            jurisdiction="International (IEEE)",
            year=1998,
            description="Software requirements specification standard"
        ),
        LegalFramework(
            name="IEEE 12207",
            full_title="IEEE 12207-2017 Software Life Cycle Processes",
            jurisdiction="International (IEEE)",
            year=2017,
            description="Software development life cycle processes"
        ),
        
        # Human Rights
        LegalFramework(
            name="UDHR",
            full_title="Universal Declaration of Human Rights",
            jurisdiction="International (UN)",
            year=1948,
            articles=["Article 27"],
            description="Everyone has right to benefit from scientific progress",
            url="https://www.un.org/en/universal-declaration-human-rights/"
        ),
        LegalFramework(
            name="ICESCR",
            full_title="International Covenant on Economic, Social and Cultural Rights",
            jurisdiction="International (UN)",
            year=1966,
            articles=["Article 15"],
            description="Right to benefit from scientific progress and protection of authorship",
            url="https://www.ohchr.org/en/instruments-mechanisms/instruments/international-covenant-economic-social-and-cultural-rights"
        ),
    ]
    
    # ═══════════════════════════════════════════════════════════════════════
    # ENHANCED CONTRIBUTIONS (Rafael Melo Reis)
    # ═══════════════════════════════════════════════════════════════════════
    
    ENHANCED_AUTHOR = Author(
        name="Rafael Melo Reis",
        affiliation="Instituto Rafael",
        email="202743211+rafaelmeloreisnovo@users.noreply.github.com",
        contribution="Original AI architecture, cognitive optimizations, ethical framework",
        year=2025
    )
    
    ENHANCED_CONTRIBUTIONS = [
        "Matrix-based unification of tensor operations",
        "Low-level optimization for computational efficiency",
        "Interoperability framework across versions and platforms",
        "Advanced mitigation strategies for numerical stability",
        "Cognitive-heuristic optimization algorithms (80+ enhancements)",
        "Temporal-invariant calculation structures",
        "Integrated ethical framework (CientiEspiritual)",
        "ESTADO FRACTAL HAJA governance model",
        "Vector→Vector AI paradigm",
        "Adaptive precision and resource management",
        "Predictive caching and pattern recognition",
        "Multi-dimensional error mitigation",
        "Fractal-based computational structures",
    ]
    
    PHILOSOPHICAL_FOUNDATION = {
        "core": "VAZIO → VERBO → CHEIO → RETRO",
        "translation": "Empty → Action → Full → Feedback",
        "motto": "Haja Lux, Haja Etica (Let there be light, let there be ethics)",
        "framework": "CientiEspiritual (Scientific-Spiritual integration)",
        "governance": "ESTADO FRACTAL HAJA",
        "institution": "Instituto Rafael",
    }
    
    @classmethod
    def get_header_text(cls, module_name: str, purpose: str, 
                       created_date: Optional[str] = None) -> str:
        """
        Generate comprehensive header text for a module.
        
        Args:
            module_name: Name of the module
            purpose: Brief description of module purpose
            created_date: ISO date string (defaults to today)
        
        Returns:
            Formatted header text with all attributions
        """
        if created_date is None:
            created_date = datetime.now().strftime("%Y-%m-%d")
        
        header = f"""#!/usr/bin/env python3
# -*- coding: utf-8 -*-
\"\"\"
{'═' * 79}
{module_name.upper()}
{'═' * 79}

BIBLIOGRAPHIC ORIGIN AND LEGISLATIVE FRAMEWORK (IMMUTABLE):
{'━' * 79}

PRIMARY AUTHORSHIP AND INTELLECTUAL FOUNDATIONS:
┌{'─' * 77}┐
│ Original Mathematical Foundations:                                        │
"""
        
        # Add primary publications
        for i, pub in enumerate(cls.PRIMARY_PUBLICATIONS, 1):
            authors_str = ", ".join(pub.authors)
            header += f"│ {i}. {authors_str:<72} │\n"
            
            title_line = f"│    \"{pub.title}\""
            padding = " " * max(0, 75 - len(title_line))
            header += title_line + padding + "│\n"
            
            venue_line = f"│    {pub.venue}, {pub.year}"
            padding = " " * max(0, 75 - len(venue_line))
            header += venue_line + padding + "│\n"
            
            if pub.doi:
                doi_line = f"│    DOI: {pub.doi}"
                padding = " " * max(0, 75 - len(doi_line))
                header += doi_line + padding + "│\n"
            if pub.isbn:
                isbn_line = f"│    ISBN: {pub.isbn}"
                padding = " " * max(0, 75 - len(isbn_line))
                header += isbn_line + padding + "│\n"
            
            found_line = f"│    Foundation: {pub.contribution_type}"
            padding = " " * max(0, 75 - len(found_line))
            header += found_line + padding + "│\n"
            header += "│                                                                           │\n"
        
        header += f"""└{'─' * 77}┘

LEGISLATIVE AND REGULATORY COMPLIANCE (BEYOND MINIMUM REQUIREMENTS):
┌{'─' * 77}┐
│ International Copyright Framework:                                        │
"""
        
        # Add key legal frameworks
        copyright_frameworks = [f for f in cls.LEGAL_FRAMEWORKS if "Copyright" in f.full_title or "Berne" in f.name or "WIPO" in f.name or "TRIPS" in f.name]
        for framework in copyright_frameworks:
            name_line = f"│ • {framework.name}: {framework.full_title[:60]}"
            padding = " " * max(0, 75 - len(name_line))
            header += name_line + padding + "│\n"
            
            if framework.articles:
                articles_str = ', '.join(framework.articles)
                articles_line = f"│   {articles_str}"
                padding = " " * max(0, 75 - len(articles_line))
                header += articles_line + padding + "│\n"
        
        header += f"""│                                                                           │
│ AI Ethics and Governance:                                                 │
"""
        
        ai_frameworks = [f for f in cls.LEGAL_FRAMEWORKS if "AI" in f.full_title or "UNESCO" in f.name or "OECD" in f.name]
        for framework in ai_frameworks:
            desc = framework.description[:60] if framework.description else ''
            fw_line = f"│ • {framework.name}: {desc}"
            padding = " " * max(0, 75 - len(fw_line))
            header += fw_line + padding + "│\n"
        
        header += f"""│                                                                           │
│ Data Protection and Privacy:                                              │
│ • GDPR (EU) 2016/679 - Privacy by design, data protection                │
│ • LGPD (Brazil) 2018 - Data protection and privacy                       │
│                                                                           │
│ Quality and Security Standards:                                           │
│ • ISO 9001:2015, ISO 27001:2022, ISO 25010:2011                         │
│ • NIST Cybersecurity Framework, NIST SP 800-53                          │
│                                                                           │
│ Human Rights Foundation:                                                  │
│ • UDHR Article 27 - Right to benefit from scientific progress           │
│ • ICESCR Article 15 - Protection of authorship                          │
└{'─' * 77}┘

ENHANCED AUTHORSHIP ({cls.ENHANCED_AUTHOR.name}):
┌{'─' * 77}┐
│ ORIGINAL CONTRIBUTIONS:                                                   │
"""
        
        for contribution in cls.ENHANCED_CONTRIBUTIONS[:10]:  # Show first 10
            header += f"│ • {contribution:<73} │\n"
        
        header += f"""│                                                                           │
│ PHILOSOPHICAL FOUNDATION:                                                 │
│ • {cls.PHILOSOPHICAL_FOUNDATION['core']:<71} │
│ • {cls.PHILOSOPHICAL_FOUNDATION['motto']:<71} │
│ • Framework: {cls.PHILOSOPHICAL_FOUNDATION['framework']:<60} │
│                                                                           │
│ INSTITUTIONAL AFFILIATION:                                                │
│ • {cls.PHILOSOPHICAL_FOUNDATION['institution']:<71} │
│ • {cls.PHILOSOPHICAL_FOUNDATION['governance']:<71} │
│                                                                           │
│ COPYRIGHT NOTICE:                                                         │
│ Copyright (C) 2025 {cls.ENHANCED_AUTHOR.name:<54} │
│ All Rights Reserved.                                                      │
│                                                                           │
│ DUAL LICENSE MODEL:                                                       │
│ 1. Social Inclusion License (Free) - Educational, research, non-profit   │
│ 2. Commercial SaaS License (Paid) - Requires subscription                │
│                                                                           │
│ AUTOMATIC PENALTIES for unauthorized commercial use:                     │
│ • Minimum: R$ 50,000 (BRL) or USD $10,000 per violation                 │
│ • Additional: 5% of gross revenue from unauthorized use                  │
│                                                                           │
│ See RAFAELIA_LICENSE.md for complete terms.                              │
└{'─' * 77}┘

SIGNATURE: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
GOLDEN RATIO (Φ): 1.618033988749894848204586834365638117720309179805762862135...
PHILOSOPHY: {cls.PHILOSOPHICAL_FOUNDATION['core']}

{'═' * 79}
MODULE PURPOSE: {purpose}
CREATED: {created_date}
VERSION: 1.0.0
STABILITY: Production
{'═' * 79}
\"\"\"
"""
        
        return header
    
    @classmethod
    def get_bibtex_entries(cls) -> str:
        """Generate BibTeX entries for all primary publications."""
        entries = []
        
        for pub in cls.PRIMARY_PUBLICATIONS:
            # Generate citation key
            first_author = pub.authors[0].split()[-1].lower()
            key = f"{first_author}{pub.year}"
            
            if pub.venue and "Journal" in pub.venue:
                entry_type = "article"
            elif pub.isbn:
                entry_type = "book"
            else:
                entry_type = "inproceedings"
            
            entry = f"@{entry_type}{{{key},\n"
            entry += f"  title = {{{pub.title}}},\n"
            entry += f"  author = {{{' and '.join(pub.authors)}}},\n"
            
            if entry_type == "article":
                entry += f"  journal = {{{pub.venue}}},\n"
                if pub.volume:
                    entry += f"  volume = {{{pub.volume}}},\n"
                if pub.number:
                    entry += f"  number = {{{pub.number}}},\n"
                if pub.pages:
                    entry += f"  pages = {{{pub.pages}}},\n"
            elif entry_type == "book":
                if pub.publisher:
                    entry += f"  publisher = {{{pub.publisher}}},\n"
                if pub.isbn:
                    entry += f"  isbn = {{{pub.isbn}}},\n"
            
            entry += f"  year = {{{pub.year}}},\n"
            
            if pub.doi:
                entry += f"  doi = {{{pub.doi}}},\n"
            if pub.url:
                entry += f"  url = {{{pub.url}}},\n"
            
            entry += "}\n"
            entries.append(entry)
        
        return "\n".join(entries)


# Export public interface
__all__ = [
    'Author',
    'Publication',
    'LegalFramework',
    'RafaeliaAuthorship',
]
