import React from 'react';

interface LegerRSLogoProps {
  className?: string;
  iconOnly?: boolean;
  size?: 'sm' | 'md' | 'lg' | 'xl';
  variant?: 'light' | 'dark';
}

export function LegerRSLogo({ className = '', iconOnly = false, size = 'md', variant = 'light' }: LegerRSLogoProps) {
  // Dimensions map based on size parameter
  const sizeMap = {
    sm: { icon: 'w-8 h-8', text: 'text-[14px]' },
    md: { icon: 'w-10 h-10', text: 'text-[18px]' },
    lg: { icon: 'w-16 h-16', text: 'text-[24px]' },
    xl: { icon: 'w-32 h-32', text: 'text-[36px]' },
  };

  const currentSize = sizeMap[size];

  // The beautiful pixel-perfect SVG of the LegerRS logo directly inspired by the user's attached design
  const Logomark = (
    <svg 
      viewBox="0 0 160 160" 
      className={`${currentSize.icon} select-none ${className}`}
      fill="none" 
      xmlns="http://www.w3.org/2000/svg"
    >
      <defs>
        {/* Elegant Teal to Mint gradient matching high-end fintech apps */}
        <linearGradient id="brandBlueGrad" x1="40" y1="30" x2="110" y2="100" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stopColor="#0f766e" />
          <stop offset="50%" stopColor="#0d9488" />
          <stop offset="100%" stopColor="#2dd4bf" />
        </linearGradient>

        <linearGradient id="accentTealGrad" x1="40" y1="90" x2="80" y2="90" gradientUnits="userSpaceOnUse">
          <stop offset="0%" stopColor="#0d9488" />
          <stop offset="100%" stopColor="#2dd4bf" />
        </linearGradient>

        {/* Soft shadow for premium depth */}
        <filter id="premiumGlow" x="-10%" y="-10%" width="120%" height="120%">
          <feDropShadow dx="0" dy="2" stdDeviation="3" floodOpacity="0.08" floodColor="#0f766e" />
        </filter>
      </defs>

      {/* Group with premium shadow */}
      <g filter="url(#premiumGlow)">
        
        {/* --- Background circuit tracks and nodes (Teal and Blue lines) --- */}
        
        {/* Leftmost line with circle node */}
        <circle cx="24" cy="80" r="3" fill="#0d9488" stroke="#FFFFFC" strokeWidth="0.5" />
        <path d="M 24,80 C 42,78 44,65 44,52" stroke="#80cbc4" strokeWidth="1" strokeLinecap="round" />

        {/* Diagonal square track rising from L bottom up-right with cascading square nodes */}
        <path d="M 64,88 L 84,42" stroke="#80cbc4" strokeWidth="0.8" strokeDasharray="3 2" />
        
        {/* Square diamond nodes (rotated 45deg) */}
        <g transform="translate(86, 38) rotate(45)">
          <rect x="-3" y="-3" width="6" height="6" fill="#2dd4bf" stroke="#FFFFFC" strokeWidth="0.5" />
        </g>
        <g transform="translate(76, 52) rotate(45)">
          <rect x="-2" y="-2" width="4" height="4" fill="#80cbc4" stroke="#FFFFFC" strokeWidth="0.5" />
        </g>
        <g transform="translate(68, 64) rotate(45)">
          <rect x="-2" y="-2" width="4" height="4" fill="#80cbc4" stroke="#FFFFFC" strokeWidth="0.5" />
        </g>
        <g transform="translate(62, 74) rotate(45)">
          <rect x="-2" y="-2" width="4" height="4" fill="#80cbc4" stroke="#FFFFFC" strokeWidth="0.5" />
        </g>

        {/* Rightmost node pointing into the R */}
        <circle cx="128" cy="62" r="3" fill="#2dd4bf" stroke="#FFFFFC" strokeWidth="0.5" />
        <path d="M 128,62 C 108,61 104,74 96,62" stroke="#80cbc4" strokeWidth="1" strokeLinecap="round" />
        
        {/* Node nested inside right loop */}
        <circle cx="106" cy="68" r="1.5" fill="#0f766e" />
        <path d="M 106,68 C 114,64 118,60 120,54" stroke="#80cbc4" strokeWidth="0.8" />


        {/* --- Central Main Stylized Letter Shapes "L" & "R" --- */}
        
        {/* LEFT L-BOWL: Thick curving letterform */}
        <path 
          d="M 33,48 L 33,74 C 33,96 52,94 59,85" 
          stroke="url(#brandBlueGrad)" 
          strokeWidth="11" 
          strokeLinecap="round" 
          strokeLinejoin="round" 
        />
        
        {/* Fine inner accent cut inside the L loop */}
        <path 
          d="M 38,48 L 38,72 Q 38,84 48,84" 
          stroke="#FFFFFC" 
          strokeWidth="1" 
          strokeOpacity="0.25" 
          strokeLinecap="round" 
        />

        {/* RIGHT R-LOOP & LEG: Flowing into a tech upward/downward arrow form */}
        <path 
          d="M 64,88 C 68,48 106,44 106,68 C 106,85 86,76 96,96 L 118,102" 
          stroke="url(#brandBlueGrad)" 
          strokeWidth="11" 
          strokeLinecap="round" 
          strokeLinejoin="round" 
        />

        {/* Inside cut of the R shape */}
        <path 
          d="M 75,76 Q 78,54 94,54 C 100,54 100,64 94,70" 
          stroke="#FFFFFC" 
          strokeWidth="1.2" 
          strokeOpacity="0.25" 
          strokeLinecap="round" 
        />

        {/* Arrow tip indicator on the foot of the R */}
        <path 
          d="M 108,82 L 118,102 M 98,90 L 118,102" 
          stroke="url(#brandBlueGrad)" 
          strokeWidth="4" 
          strokeLinecap="round" 
          strokeLinejoin="round" 
        />


        {/* --- Bottom Base Capsule/Foundation --- */}
        
        {/* Base horizontal line */}
        <path d="M 28,112 L 126,112" stroke="#0f766e" strokeWidth="1.2" strokeLinecap="round" />
        <circle cx="28" cy="112" r="1.5" fill="#0f766e" />
        <circle cx="126" cy="112" r="1.5" fill="#0f766e" />

        {/* Divided capsule on the baseline representing assets partition */}
        <g>
          <clipPath id="capsuleClip">
            <rect x="42" y="102" width="62" height="15" rx="7.5" fill="none" />
          </clipPath>
          
          <g clipPath="url(#capsuleClip)">
            {/* Left side green-teal segment of capsule */}
            <rect x="40" y="100" width="34" height="20" fill="url(#accentTealGrad)" />
            {/* Right side dark-blue segment of capsule */}
            <rect x="73" y="100" width="34" height="20" fill="#0d9488" />
            {/* Soft inner shadow separator */}
            <line x1="73.5" y1="102" x2="73.5" y2="117" stroke="#0f766e" strokeWidth="1.2" strokeOpacity="0.15" />
          </g>
          {/* Subtle outline of capsule */}
          <rect x="42" y="102" width="62" height="15" rx="7.5" stroke="#0f766e" strokeWidth="1" strokeOpacity="0.3" />
        </g>

        {/* Capsule little interactive center dot */}
        <circle cx="56" cy="109.5" r="1.5" fill="#0f766e" fillOpacity="0.6" />

      </g>
    </svg>
  );

  if (iconOnly) {
    return Logomark;
  }

  // Double columns logo text: Navy blue "Leger" + Slate gray "RS"
  return (
    <div className="flex items-center gap-2.5 select-none font-sans">
      <div className="relative flex-shrink-0 flex items-center justify-center">
        {Logomark}
      </div>
      <div>
        <div className="flex items-baseline tracking-tight">
          <span className={`font-extrabold text-[17px] tracking-tight leading-none ${variant === 'dark' ? 'text-white' : 'text-slate-800'}`}>Leger</span>
          <span className={`font-medium text-[17px] tracking-tight leading-none ${variant === 'dark' ? 'text-indigo-200' : 'text-[#0d9488]'}`}>RS</span>
        </div>
        <span className={`text-[10px] font-bold mt-0.5 block tracking-wider uppercase ${variant === 'dark' ? 'text-slate-400' : 'text-slate-400'}`}>多币资产账簿</span>
      </div>
    </div>
  );
}
