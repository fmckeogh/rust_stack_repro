#![no_std]
#![allow(non_snake_case)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_doc_comments)]
#![allow(non_upper_case_globals)]
//! BOREALIS GENERATED FILE
extern crate alloc;
use AArch64_S2ApplyOutputPerms::*;
use S2DecodeMemAttrs::*;
use AArch64_SS2OutputPASpace::*;
use AArch64_ContiguousBit::*;
use AArch64_S2ApplyFWBMemAttrs::*;
use AArch64_LeafBase::*;
use common::*;
pub fn AArch64_S2NextWalkStateLeaf<T: Tracer>(
    state: &mut State,
    tracer: &T,
    currentstate: ProductType96e7acababe246a1,
    ss: u32,
    walkparams: ProductTypeb05ce25a107f0c5e,
    ipa: ProductTypece7c66ccb2cab13e,
    descriptor: Bits,
) -> ProductType96e7acababe246a1 {
    #[derive(Default)]
    struct FunctionState {
        nextstate: ProductType96e7acababe246a1,
        s2_fnxs: bool,
        s2_sh: u8,
        ns: bool,
        baseaddress: ProductTypeda0231e9dc169f81,
        ga_14401: ProductTypeda0231e9dc169f81,
        s2_attr: u8,
        currentstate: ProductType96e7acababe246a1,
        ss: u32,
        walkparams: ProductTypeb05ce25a107f0c5e,
        ipa: ProductTypece7c66ccb2cab13e,
        descriptor: Bits,
    }
    let fn_state = FunctionState {
        currentstate,
        ss,
        walkparams,
        ipa,
        descriptor,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_0_0: read-var ss:u32
        let s_0_0: u32 = fn_state.ss;
        // C s_0_1: const #3u : u32
        let s_0_1: u32 = 3;
        // D s_0_2: cmp-eq s_0_0 s_0_1
        let s_0_2: bool = ((s_0_0) == (s_0_1));
        // N s_0_3: branch s_0_2 b27 b1
        if s_0_2 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_1_0: read-var ss:u32
        let s_1_0: u32 = fn_state.ss;
        // C s_1_1: const #2u : u32
        let s_1_1: u32 = 2;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b20 b2
        if s_1_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_2_0: const #0u : u32
        let s_2_0: u32 = 0;
        // D s_2_1: write-var baseaddress.1 <= s_2_0
        fn_state.baseaddress._1 = s_2_0;
        // N s_2_2: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_3_0: read-var walkparams.2:struct
        let s_3_0: bool = fn_state.walkparams._2;
        // D s_3_1: read-var walkparams.3:struct
        let s_3_1: bool = fn_state.walkparams._3;
        // D s_3_2: read-var walkparams.26:struct
        let s_3_2: u32 = fn_state.walkparams._26;
        // D s_3_3: read-var currentstate.6:struct
        let s_3_3: i128 = fn_state.currentstate._6;
        // D s_3_4: read-var descriptor:bv
        let s_3_4: Bits = fn_state.descriptor;
        // D s_3_5: call AArch64_LeafBase(s_3_4, s_3_0, s_3_1, s_3_2, s_3_3)
        let s_3_5: u64 = AArch64_LeafBase(
            state,
            tracer,
            s_3_4,
            s_3_0,
            s_3_1,
            s_3_2,
            s_3_3,
        );
        // D s_3_6: write-var baseaddress.0 <= s_3_5
        fn_state.baseaddress._0 = s_3_5;
        // C s_3_7: const #0u : u8
        let s_3_7: bool = false;
        // D s_3_8: write-var nextstate.5 <= s_3_7
        fn_state.nextstate._5 = s_3_7;
        // D s_3_9: read-var currentstate.6:struct
        let s_3_9: i128 = fn_state.currentstate._6;
        // D s_3_10: write-var nextstate.6 <= s_3_9
        fn_state.nextstate._6 = s_3_9;
        // D s_3_11: read-var baseaddress:struct
        let s_3_11: ProductTypeda0231e9dc169f81 = fn_state.baseaddress;
        // D s_3_12: write-var nextstate.0 <= s_3_11
        fn_state.nextstate._0 = s_3_11;
        // D s_3_13: read-var descriptor:bv
        let s_3_13: Bits = fn_state.descriptor;
        // D s_3_14: read-var walkparams:struct
        let s_3_14: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_3_15: call AArch64_S2ApplyOutputPerms(s_3_13, s_3_14)
        let s_3_15: ProductTypebf05c51f33174538 = AArch64_S2ApplyOutputPerms(
            state,
            tracer,
            s_3_13,
            s_3_14,
        );
        // D s_3_16: write-var nextstate.9 <= s_3_15
        fn_state.nextstate._9 = s_3_15;
        // C s_3_17: const #2s : i
        let s_3_17: i128 = 2;
        // D s_3_18: read-var descriptor:bv
        let s_3_18: Bits = fn_state.descriptor;
        // C s_3_19: const #1s : i64
        let s_3_19: i64 = 1;
        // C s_3_20: cast zx s_3_19 -> i
        let s_3_20: i128 = (i128::try_from(s_3_19).unwrap());
        // C s_3_21: const #3s : i
        let s_3_21: i128 = 3;
        // C s_3_22: add s_3_21 s_3_20
        let s_3_22: i128 = (s_3_21 + s_3_20);
        // D s_3_23: bit-extract s_3_18 s_3_17 s_3_22
        let s_3_23: Bits = (Bits::new(
            ((s_3_18) >> (s_3_17)).value(),
            u16::try_from(s_3_22).unwrap(),
        ));
        // D s_3_24: cast reint s_3_23 -> u8
        let s_3_24: u8 = (s_3_23.value() as u8);
        // D s_3_25: write-var s2_attr <= s_3_24
        fn_state.s2_attr = s_3_24;
        // D s_3_26: read-var walkparams.3:struct
        let s_3_26: bool = fn_state.walkparams._3;
        // D s_3_27: cast zx s_3_26 -> bv
        let s_3_27: Bits = Bits::new(s_3_26 as u128, 1u16);
        // C s_3_28: const #1u : u8
        let s_3_28: bool = true;
        // C s_3_29: cast zx s_3_28 -> bv
        let s_3_29: Bits = Bits::new(s_3_28 as u128, 1u16);
        // D s_3_30: cmp-eq s_3_27 s_3_29
        let s_3_30: bool = ((s_3_27) == (s_3_29));
        // N s_3_31: branch s_3_30 b19 b4
        if s_3_30 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_4_0: read-var descriptor:bv
        let s_4_0: Bits = fn_state.descriptor;
        // D s_4_1: size-of s_4_0
        let s_4_1: u16 = s_4_0.length();
        // D s_4_2: cast zx s_4_1 -> i
        let s_4_2: i128 = (i128::try_from(s_4_1).unwrap());
        // C s_4_3: const #9s : i
        let s_4_3: i128 = 9;
        // D s_4_4: cmp-lt s_4_3 s_4_2
        let s_4_4: bool = ((s_4_3) < (s_4_2));
        // N s_4_5: assert s_4_4
        let s_4_5: () = assert!(s_4_4);
        // C s_4_6: const #8s : i
        let s_4_6: i128 = 8;
        // D s_4_7: read-var descriptor:bv
        let s_4_7: Bits = fn_state.descriptor;
        // C s_4_8: const #1s : i64
        let s_4_8: i64 = 1;
        // C s_4_9: cast zx s_4_8 -> i
        let s_4_9: i128 = (i128::try_from(s_4_8).unwrap());
        // C s_4_10: const #1s : i
        let s_4_10: i128 = 1;
        // C s_4_11: add s_4_10 s_4_9
        let s_4_11: i128 = (s_4_10 + s_4_9);
        // D s_4_12: bit-extract s_4_7 s_4_6 s_4_11
        let s_4_12: Bits = (Bits::new(
            ((s_4_7) >> (s_4_6)).value(),
            u16::try_from(s_4_11).unwrap(),
        ));
        // D s_4_13: cast reint s_4_12 -> u8
        let s_4_13: u8 = (s_4_12.value() as u8);
        // D s_4_14: write-var s2_sh <= s_4_13
        fn_state.s2_sh = s_4_13;
        // N s_4_15: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_5_0: const #11s : i
        let s_5_0: i128 = 11;
        // D s_5_1: read-var descriptor:bv
        let s_5_1: Bits = fn_state.descriptor;
        // C s_5_2: const #1u : u64
        let s_5_2: u64 = 1;
        // D s_5_3: bit-extract s_5_1 s_5_0 s_5_2
        let s_5_3: Bits = (Bits::new(
            ((s_5_1) >> (s_5_0)).value(),
            u16::try_from(s_5_2).unwrap(),
        ));
        // D s_5_4: cast reint s_5_3 -> u8
        let s_5_4: bool = ((s_5_3.value()) != 0);
        // C s_5_5: const #0s : i
        let s_5_5: i128 = 0;
        // C s_5_6: const #0u : u64
        let s_5_6: u64 = 0;
        // D s_5_7: cast zx s_5_4 -> u64
        let s_5_7: u64 = (s_5_4 as u64);
        // C s_5_8: const #1u : u64
        let s_5_8: u64 = 1;
        // D s_5_9: and s_5_7 s_5_8
        let s_5_9: u64 = ((s_5_7) & (s_5_8));
        // D s_5_10: cmp-eq s_5_9 s_5_8
        let s_5_10: bool = ((s_5_9) == (s_5_8));
        // D s_5_11: lsl s_5_7 s_5_5
        let s_5_11: u64 = s_5_7 << s_5_5;
        // D s_5_12: or s_5_6 s_5_11
        let s_5_12: u64 = ((s_5_6) | (s_5_11));
        // D s_5_13: cmpl s_5_11
        let s_5_13: u64 = !s_5_11;
        // D s_5_14: and s_5_6 s_5_13
        let s_5_14: u64 = ((s_5_6) & (s_5_13));
        // D s_5_15: select s_5_10 s_5_12 s_5_14
        let s_5_15: u64 = if s_5_10 { s_5_12 } else { s_5_14 };
        // D s_5_16: cast trunc s_5_15 -> u8
        let s_5_16: bool = ((s_5_15) != 0);
        // D s_5_17: write-var s2_fnxs <= s_5_16
        fn_state.s2_fnxs = s_5_16;
        // D s_5_18: read-var walkparams.6:struct
        let s_5_18: bool = fn_state.walkparams._6;
        // D s_5_19: cast zx s_5_18 -> bv
        let s_5_19: Bits = Bits::new(s_5_18 as u128, 1u16);
        // C s_5_20: const #1u : u8
        let s_5_20: bool = true;
        // C s_5_21: cast zx s_5_20 -> bv
        let s_5_21: Bits = Bits::new(s_5_20 as u128, 1u16);
        // D s_5_22: cmp-eq s_5_19 s_5_21
        let s_5_22: bool = ((s_5_19) == (s_5_21));
        // N s_5_23: branch s_5_22 b16 b6
        if s_5_22 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_6_0: const #1u : u8
        let s_6_0: bool = true;
        // D s_6_1: read-var s2_attr:u8
        let s_6_1: u8 = fn_state.s2_attr;
        // D s_6_2: read-var s2_sh:u8
        let s_6_2: u8 = fn_state.s2_sh;
        // D s_6_3: call S2DecodeMemAttrs(s_6_1, s_6_2, s_6_0)
        let s_6_3: ProductTypef170cab34335b70c = S2DecodeMemAttrs(
            state,
            tracer,
            s_6_1,
            s_6_2,
            s_6_0,
        );
        // D s_6_4: write-var nextstate.7 <= s_6_3
        fn_state.nextstate._7 = s_6_3;
        // D s_6_5: read-var s2_fnxs:u8
        let s_6_5: bool = fn_state.s2_fnxs;
        // D s_6_6: cast zx s_6_5 -> bv
        let s_6_6: Bits = Bits::new(s_6_5 as u128, 1u16);
        // D s_6_7: not s_6_6
        let s_6_7: Bits = !s_6_6;
        // D s_6_8: cast reint s_6_7 -> u8
        let s_6_8: bool = ((s_6_7.value()) != 0);
        // D s_6_9: write-var nextstate.7.7 <= s_6_8
        fn_state.nextstate._7._7 = s_6_8;
        // D s_6_10: read-var s2_attr:u8
        let s_6_10: u8 = fn_state.s2_attr;
        // D s_6_11: cast zx s_6_10 -> bv
        let s_6_11: Bits = Bits::new(s_6_10 as u128, 4u16);
        // C s_6_12: const #4u : u8
        let s_6_12: u8 = 4;
        // C s_6_13: cast zx s_6_12 -> bv
        let s_6_13: Bits = Bits::new(s_6_12 as u128, 4u16);
        // D s_6_14: cmp-eq s_6_11 s_6_13
        let s_6_14: bool = ((s_6_11) == (s_6_13));
        // N s_6_15: branch s_6_14 b15 b7
        if s_6_14 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_7_0: const #0u : u8
        let s_7_0: bool = false;
        // D s_7_1: write-var nextstate.9.11 <= s_7_0
        fn_state.nextstate._9._11 = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_8_0: read-var walkparams.26:struct
        let s_8_0: u32 = fn_state.walkparams._26;
        // D s_8_1: read-var walkparams.2:struct
        let s_8_1: bool = fn_state.walkparams._2;
        // D s_8_2: read-var currentstate.6:struct
        let s_8_2: i128 = fn_state.currentstate._6;
        // D s_8_3: read-var descriptor:bv
        let s_8_3: Bits = fn_state.descriptor;
        // D s_8_4: call AArch64_ContiguousBit(s_8_0, s_8_1, s_8_2, s_8_3)
        let s_8_4: bool = AArch64_ContiguousBit(
            state,
            tracer,
            s_8_0,
            s_8_1,
            s_8_2,
            s_8_3,
        );
        // D s_8_5: write-var nextstate.1 <= s_8_4
        fn_state.nextstate._1 = s_8_4;
        // D s_8_6: read-var walkparams.2:struct
        let s_8_6: bool = fn_state.walkparams._2;
        // D s_8_7: cast zx s_8_6 -> bv
        let s_8_7: Bits = Bits::new(s_8_6 as u128, 1u16);
        // C s_8_8: const #1u : u8
        let s_8_8: bool = true;
        // C s_8_9: cast zx s_8_8 -> bv
        let s_8_9: Bits = Bits::new(s_8_8 as u128, 1u16);
        // D s_8_10: cmp-eq s_8_7 s_8_9
        let s_8_10: bool = ((s_8_7) == (s_8_9));
        // N s_8_11: branch s_8_10 b14 b9
        if s_8_10 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_9_0: read-var walkparams.0:struct
        let s_9_0: bool = fn_state.walkparams._0;
        // D s_9_1: cast zx s_9_0 -> bv
        let s_9_1: Bits = Bits::new(s_9_0 as u128, 1u16);
        // C s_9_2: const #1u : u8
        let s_9_2: bool = true;
        // C s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // D s_9_4: cmp-eq s_9_1 s_9_3
        let s_9_4: bool = ((s_9_1) == (s_9_3));
        // N s_9_5: branch s_9_4 b13 b10
        if s_9_4 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var nextstate.11 <= s_10_0
        fn_state.nextstate._11 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // N s_11_0: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_12_0: read-var nextstate:struct
        let s_12_0: ProductType96e7acababe246a1 = fn_state.nextstate;
        // N s_12_1: return s_12_0
        return s_12_0;
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_13_0: read-var descriptor:bv
        let s_13_0: Bits = fn_state.descriptor;
        // D s_13_1: size-of s_13_0
        let s_13_1: u16 = s_13_0.length();
        // D s_13_2: cast zx s_13_1 -> i
        let s_13_2: i128 = (i128::try_from(s_13_1).unwrap());
        // C s_13_3: const #58s : i
        let s_13_3: i128 = 58;
        // D s_13_4: cmp-lt s_13_3 s_13_2
        let s_13_4: bool = ((s_13_3) < (s_13_2));
        // N s_13_5: assert s_13_4
        let s_13_5: () = assert!(s_13_4);
        // C s_13_6: const #58s : i
        let s_13_6: i128 = 58;
        // D s_13_7: read-var descriptor:bv
        let s_13_7: Bits = fn_state.descriptor;
        // C s_13_8: const #1u : u64
        let s_13_8: u64 = 1;
        // D s_13_9: bit-extract s_13_7 s_13_6 s_13_8
        let s_13_9: Bits = (Bits::new(
            ((s_13_7) >> (s_13_6)).value(),
            u16::try_from(s_13_8).unwrap(),
        ));
        // D s_13_10: cast reint s_13_9 -> u8
        let s_13_10: bool = ((s_13_9.value()) != 0);
        // C s_13_11: const #0s : i
        let s_13_11: i128 = 0;
        // C s_13_12: const #0u : u64
        let s_13_12: u64 = 0;
        // D s_13_13: cast zx s_13_10 -> u64
        let s_13_13: u64 = (s_13_10 as u64);
        // C s_13_14: const #1u : u64
        let s_13_14: u64 = 1;
        // D s_13_15: and s_13_13 s_13_14
        let s_13_15: u64 = ((s_13_13) & (s_13_14));
        // D s_13_16: cmp-eq s_13_15 s_13_14
        let s_13_16: bool = ((s_13_15) == (s_13_14));
        // D s_13_17: lsl s_13_13 s_13_11
        let s_13_17: u64 = s_13_13 << s_13_11;
        // D s_13_18: or s_13_12 s_13_17
        let s_13_18: u64 = ((s_13_12) | (s_13_17));
        // D s_13_19: cmpl s_13_17
        let s_13_19: u64 = !s_13_17;
        // D s_13_20: and s_13_12 s_13_19
        let s_13_20: u64 = ((s_13_12) & (s_13_19));
        // D s_13_21: select s_13_16 s_13_18 s_13_20
        let s_13_21: u64 = if s_13_16 { s_13_18 } else { s_13_20 };
        // D s_13_22: cast trunc s_13_21 -> u8
        let s_13_22: bool = ((s_13_21) != 0);
        // D s_13_23: write-var nextstate.11 <= s_13_22
        fn_state.nextstate._11 = s_13_22;
        // N s_13_24: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_14_0: read-var descriptor:bv
        let s_14_0: Bits = fn_state.descriptor;
        // D s_14_1: size-of s_14_0
        let s_14_1: u16 = s_14_0.length();
        // D s_14_2: cast zx s_14_1 -> i
        let s_14_2: i128 = (i128::try_from(s_14_1).unwrap());
        // C s_14_3: const #114s : i
        let s_14_3: i128 = 114;
        // D s_14_4: cmp-lt s_14_3 s_14_2
        let s_14_4: bool = ((s_14_3) < (s_14_2));
        // N s_14_5: assert s_14_4
        let s_14_5: () = assert!(s_14_4);
        // C s_14_6: const #114s : i
        let s_14_6: i128 = 114;
        // D s_14_7: read-var descriptor:bv
        let s_14_7: Bits = fn_state.descriptor;
        // C s_14_8: const #1u : u64
        let s_14_8: u64 = 1;
        // D s_14_9: bit-extract s_14_7 s_14_6 s_14_8
        let s_14_9: Bits = (Bits::new(
            ((s_14_7) >> (s_14_6)).value(),
            u16::try_from(s_14_8).unwrap(),
        ));
        // D s_14_10: cast reint s_14_9 -> u8
        let s_14_10: bool = ((s_14_9.value()) != 0);
        // C s_14_11: const #0s : i
        let s_14_11: i128 = 0;
        // C s_14_12: const #0u : u64
        let s_14_12: u64 = 0;
        // D s_14_13: cast zx s_14_10 -> u64
        let s_14_13: u64 = (s_14_10 as u64);
        // C s_14_14: const #1u : u64
        let s_14_14: u64 = 1;
        // D s_14_15: and s_14_13 s_14_14
        let s_14_15: u64 = ((s_14_13) & (s_14_14));
        // D s_14_16: cmp-eq s_14_15 s_14_14
        let s_14_16: bool = ((s_14_15) == (s_14_14));
        // D s_14_17: lsl s_14_13 s_14_11
        let s_14_17: u64 = s_14_13 << s_14_11;
        // D s_14_18: or s_14_12 s_14_17
        let s_14_18: u64 = ((s_14_12) | (s_14_17));
        // D s_14_19: cmpl s_14_17
        let s_14_19: u64 = !s_14_17;
        // D s_14_20: and s_14_12 s_14_19
        let s_14_20: u64 = ((s_14_12) & (s_14_19));
        // D s_14_21: select s_14_16 s_14_18 s_14_20
        let s_14_21: u64 = if s_14_16 { s_14_18 } else { s_14_20 };
        // D s_14_22: cast trunc s_14_21 -> u8
        let s_14_22: bool = ((s_14_21) != 0);
        // D s_14_23: write-var nextstate.11 <= s_14_22
        fn_state.nextstate._11 = s_14_22;
        // N s_14_24: jump b12
        return block_12(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_15_0: const #1u : u8
        let s_15_0: bool = true;
        // D s_15_1: write-var nextstate.9.11 <= s_15_0
        fn_state.nextstate._9._11 = s_15_0;
        // N s_15_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_16_0: read-var ipa.2:struct
        let s_16_0: ProductTypef170cab34335b70c = fn_state.ipa._2;
        // D s_16_1: read-var walkparams:struct
        let s_16_1: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_16_2: read-var descriptor:bv
        let s_16_2: Bits = fn_state.descriptor;
        // D s_16_3: call AArch64_S2ApplyFWBMemAttrs(s_16_0, s_16_1, s_16_2)
        let s_16_3: ProductTypef170cab34335b70c = AArch64_S2ApplyFWBMemAttrs(
            state,
            tracer,
            s_16_0,
            s_16_1,
            s_16_2,
        );
        // D s_16_4: write-var nextstate.7 <= s_16_3
        fn_state.nextstate._7 = s_16_3;
        // C s_16_5: const #1s : i
        let s_16_5: i128 = 1;
        // D s_16_6: read-var s2_attr:u8
        let s_16_6: u8 = fn_state.s2_attr;
        // D s_16_7: cast zx s_16_6 -> bv
        let s_16_7: Bits = Bits::new(s_16_6 as u128, 4u16);
        // C s_16_8: const #1s : i64
        let s_16_8: i64 = 1;
        // C s_16_9: cast zx s_16_8 -> i
        let s_16_9: i128 = (i128::try_from(s_16_8).unwrap());
        // C s_16_10: const #2s : i
        let s_16_10: i128 = 2;
        // C s_16_11: add s_16_10 s_16_9
        let s_16_11: i128 = (s_16_10 + s_16_9);
        // D s_16_12: bit-extract s_16_7 s_16_5 s_16_11
        let s_16_12: Bits = (Bits::new(
            ((s_16_7) >> (s_16_5)).value(),
            u16::try_from(s_16_11).unwrap(),
        ));
        // D s_16_13: cast reint s_16_12 -> u8
        let s_16_13: u8 = (s_16_12.value() as u8);
        // D s_16_14: cast zx s_16_13 -> bv
        let s_16_14: Bits = Bits::new(s_16_13 as u128, 3u16);
        // C s_16_15: const #7u : u8
        let s_16_15: u8 = 7;
        // C s_16_16: cast zx s_16_15 -> bv
        let s_16_16: Bits = Bits::new(s_16_15 as u128, 3u16);
        // D s_16_17: cmp-eq s_16_14 s_16_16
        let s_16_17: bool = ((s_16_14) == (s_16_16));
        // N s_16_18: branch s_16_17 b18 b17
        if s_16_17 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_17(state, tracer, fn_state);
        };
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var nextstate.9.11 <= s_17_0
        fn_state.nextstate._9._11 = s_17_0;
        // N s_17_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_18_0: const #1u : u8
        let s_18_0: bool = true;
        // D s_18_1: write-var nextstate.9.11 <= s_18_0
        fn_state.nextstate._9._11 = s_18_0;
        // N s_18_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_19_0: read-var walkparams.20:struct
        let s_19_0: u8 = fn_state.walkparams._20;
        // D s_19_1: write-var s2_sh <= s_19_0
        fn_state.s2_sh = s_19_0;
        // N s_19_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_20_0: read-var walkparams.2:struct
        let s_20_0: bool = fn_state.walkparams._2;
        // D s_20_1: cast zx s_20_0 -> bv
        let s_20_1: Bits = Bits::new(s_20_0 as u128, 1u16);
        // C s_20_2: const #1u : u8
        let s_20_2: bool = true;
        // C s_20_3: cast zx s_20_2 -> bv
        let s_20_3: Bits = Bits::new(s_20_2 as u128, 1u16);
        // D s_20_4: cmp-eq s_20_1 s_20_3
        let s_20_4: bool = ((s_20_1) == (s_20_3));
        // N s_20_5: branch s_20_4 b26 b21
        if s_20_4 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_21(state, tracer, fn_state);
        };
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_21_0: read-var descriptor:bv
        let s_21_0: Bits = fn_state.descriptor;
        // D s_21_1: size-of s_21_0
        let s_21_1: u16 = s_21_0.length();
        // D s_21_2: cast zx s_21_1 -> i
        let s_21_2: i128 = (i128::try_from(s_21_1).unwrap());
        // C s_21_3: const #55s : i
        let s_21_3: i128 = 55;
        // D s_21_4: cmp-lt s_21_3 s_21_2
        let s_21_4: bool = ((s_21_3) < (s_21_2));
        // N s_21_5: assert s_21_4
        let s_21_5: () = assert!(s_21_4);
        // C s_21_6: const #55s : i
        let s_21_6: i128 = 55;
        // D s_21_7: read-var descriptor:bv
        let s_21_7: Bits = fn_state.descriptor;
        // C s_21_8: const #1u : u64
        let s_21_8: u64 = 1;
        // D s_21_9: bit-extract s_21_7 s_21_6 s_21_8
        let s_21_9: Bits = (Bits::new(
            ((s_21_7) >> (s_21_6)).value(),
            u16::try_from(s_21_8).unwrap(),
        ));
        // D s_21_10: cast reint s_21_9 -> u8
        let s_21_10: bool = ((s_21_9.value()) != 0);
        // C s_21_11: const #0s : i
        let s_21_11: i128 = 0;
        // C s_21_12: const #0u : u64
        let s_21_12: u64 = 0;
        // D s_21_13: cast zx s_21_10 -> u64
        let s_21_13: u64 = (s_21_10 as u64);
        // C s_21_14: const #1u : u64
        let s_21_14: u64 = 1;
        // D s_21_15: and s_21_13 s_21_14
        let s_21_15: u64 = ((s_21_13) & (s_21_14));
        // D s_21_16: cmp-eq s_21_15 s_21_14
        let s_21_16: bool = ((s_21_15) == (s_21_14));
        // D s_21_17: lsl s_21_13 s_21_11
        let s_21_17: u64 = s_21_13 << s_21_11;
        // D s_21_18: or s_21_12 s_21_17
        let s_21_18: u64 = ((s_21_12) | (s_21_17));
        // D s_21_19: cmpl s_21_17
        let s_21_19: u64 = !s_21_17;
        // D s_21_20: and s_21_12 s_21_19
        let s_21_20: u64 = ((s_21_12) & (s_21_19));
        // D s_21_21: select s_21_16 s_21_18 s_21_20
        let s_21_21: u64 = if s_21_16 { s_21_18 } else { s_21_20 };
        // D s_21_22: cast trunc s_21_21 -> u8
        let s_21_22: bool = ((s_21_21) != 0);
        // D s_21_23: write-var ns <= s_21_22
        fn_state.ns = s_21_22;
        // N s_21_24: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_22_0: read-var ns:u8
        let s_22_0: bool = fn_state.ns;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 1u16);
        // C s_22_2: const #1u : u8
        let s_22_2: bool = true;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // N s_22_5: branch s_22_4 b25 b23
        if s_22_4 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_23_0: const #3u : u32
        let s_23_0: u32 = 3;
        // D s_23_1: write-var baseaddress.1 <= s_23_0
        fn_state.baseaddress._1 = s_23_0;
        // N s_23_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // N s_24_0: jump b3
        return block_3(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // C s_25_0: const #0u : u32
        let s_25_0: u32 = 0;
        // D s_25_1: write-var baseaddress.1 <= s_25_0
        fn_state.baseaddress._1 = s_25_0;
        // N s_25_2: jump b24
        return block_24(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_26_0: read-var descriptor:bv
        let s_26_0: Bits = fn_state.descriptor;
        // D s_26_1: size-of s_26_0
        let s_26_1: u16 = s_26_0.length();
        // D s_26_2: cast zx s_26_1 -> i
        let s_26_2: i128 = (i128::try_from(s_26_1).unwrap());
        // C s_26_3: const #127s : i
        let s_26_3: i128 = 127;
        // D s_26_4: cmp-lt s_26_3 s_26_2
        let s_26_4: bool = ((s_26_3) < (s_26_2));
        // N s_26_5: assert s_26_4
        let s_26_5: () = assert!(s_26_4);
        // C s_26_6: const #127s : i
        let s_26_6: i128 = 127;
        // D s_26_7: read-var descriptor:bv
        let s_26_7: Bits = fn_state.descriptor;
        // C s_26_8: const #1u : u64
        let s_26_8: u64 = 1;
        // D s_26_9: bit-extract s_26_7 s_26_6 s_26_8
        let s_26_9: Bits = (Bits::new(
            ((s_26_7) >> (s_26_6)).value(),
            u16::try_from(s_26_8).unwrap(),
        ));
        // D s_26_10: cast reint s_26_9 -> u8
        let s_26_10: bool = ((s_26_9.value()) != 0);
        // C s_26_11: const #0s : i
        let s_26_11: i128 = 0;
        // C s_26_12: const #0u : u64
        let s_26_12: u64 = 0;
        // D s_26_13: cast zx s_26_10 -> u64
        let s_26_13: u64 = (s_26_10 as u64);
        // C s_26_14: const #1u : u64
        let s_26_14: u64 = 1;
        // D s_26_15: and s_26_13 s_26_14
        let s_26_15: u64 = ((s_26_13) & (s_26_14));
        // D s_26_16: cmp-eq s_26_15 s_26_14
        let s_26_16: bool = ((s_26_15) == (s_26_14));
        // D s_26_17: lsl s_26_13 s_26_11
        let s_26_17: u64 = s_26_13 << s_26_11;
        // D s_26_18: or s_26_12 s_26_17
        let s_26_18: u64 = ((s_26_12) | (s_26_17));
        // D s_26_19: cmpl s_26_17
        let s_26_19: u64 = !s_26_17;
        // D s_26_20: and s_26_12 s_26_19
        let s_26_20: u64 = ((s_26_12) & (s_26_19));
        // D s_26_21: select s_26_16 s_26_18 s_26_20
        let s_26_21: u64 = if s_26_16 { s_26_18 } else { s_26_20 };
        // D s_26_22: cast trunc s_26_21 -> u8
        let s_26_22: bool = ((s_26_21) != 0);
        // D s_26_23: write-var ns <= s_26_22
        fn_state.ns = s_26_22;
        // N s_26_24: jump b22
        return block_22(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductType96e7acababe246a1 {
        // D s_27_0: read-var ipa.3:struct
        let s_27_0: ProductTypeda0231e9dc169f81 = fn_state.ipa._3;
        // D s_27_1: write-var ga#14401 <= s_27_0
        fn_state.ga_14401 = s_27_0;
        // D s_27_2: read-var ga#14401.1:struct
        let s_27_2: u32 = fn_state.ga_14401._1;
        // D s_27_3: read-var walkparams:struct
        let s_27_3: ProductTypeb05ce25a107f0c5e = fn_state.walkparams;
        // D s_27_4: call AArch64_SS2OutputPASpace(s_27_3, s_27_2)
        let s_27_4: u32 = AArch64_SS2OutputPASpace(state, tracer, s_27_3, s_27_2);
        // D s_27_5: write-var baseaddress.1 <= s_27_4
        fn_state.baseaddress._1 = s_27_4;
        // N s_27_6: jump b3
        return block_3(state, tracer, fn_state);
    }
}
