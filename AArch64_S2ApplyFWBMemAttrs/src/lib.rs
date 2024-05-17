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
use S2CombineS1Shareability::*;
use S2CombineS1Device::*;
use EffectiveShareability::*;
use DecodeDevice::*;
use DecodeShareability::*;
use S2MemTagType::*;
use common::*;
pub fn AArch64_S2ApplyFWBMemAttrs<T: Tracer>(
    state: &mut State,
    tracer: &T,
    s1_memattrs: ProductTypef170cab34335b70c,
    walkparams: ProductTypeb05ce25a107f0c5e,
    descriptor: Bits,
) -> ProductTypef170cab34335b70c {
    #[derive(Default)]
    struct FunctionState {
        ga_14250: ProductType183e6678e5239c85,
        s2_deviceshadow_314: u32,
        s2_fnxs: bool,
        s2_sh: u8,
        gs_19054: bool,
        memattrs: ProductTypef170cab34335b70c,
        gs_19053: bool,
        gs_19070: bool,
        cacheability_attr: ProductType183e6678e5239c85,
        ga_14256: ProductType183e6678e5239c85,
        ga_14252: ProductType183e6678e5239c85,
        ga_14249: ProductType183e6678e5239c85,
        ga_14245: ProductType183e6678e5239c85,
        s2_attr: u8,
        ga_14257: ProductType183e6678e5239c85,
        s1_memattrs: ProductTypef170cab34335b70c,
        walkparams: ProductTypeb05ce25a107f0c5e,
        descriptor: Bits,
    }
    let fn_state = FunctionState {
        s1_memattrs,
        walkparams,
        descriptor,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_0_0: const #2s : i
        let s_0_0: i128 = 2;
        // D s_0_1: read-var descriptor:bv
        let s_0_1: Bits = fn_state.descriptor;
        // C s_0_2: const #1s : i64
        let s_0_2: i64 = 1;
        // C s_0_3: cast zx s_0_2 -> i
        let s_0_3: i128 = (i128::try_from(s_0_2).unwrap());
        // C s_0_4: const #3s : i
        let s_0_4: i128 = 3;
        // C s_0_5: add s_0_4 s_0_3
        let s_0_5: i128 = (s_0_4 + s_0_3);
        // D s_0_6: bit-extract s_0_1 s_0_0 s_0_5
        let s_0_6: Bits = (Bits::new(
            ((s_0_1) >> (s_0_0)).value(),
            u16::try_from(s_0_5).unwrap(),
        ));
        // D s_0_7: cast reint s_0_6 -> u8
        let s_0_7: u8 = (s_0_6.value() as u8);
        // D s_0_8: write-var s2_attr <= s_0_7
        fn_state.s2_attr = s_0_7;
        // D s_0_9: read-var walkparams.3:struct
        let s_0_9: bool = fn_state.walkparams._3;
        // D s_0_10: cast zx s_0_9 -> bv
        let s_0_10: Bits = Bits::new(s_0_9 as u128, 1u16);
        // C s_0_11: const #1u : u8
        let s_0_11: bool = true;
        // C s_0_12: cast zx s_0_11 -> bv
        let s_0_12: Bits = Bits::new(s_0_11 as u128, 1u16);
        // D s_0_13: cmp-eq s_0_10 s_0_12
        let s_0_13: bool = ((s_0_10) == (s_0_12));
        // N s_0_14: branch s_0_13 b33 b1
        if s_0_13 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_1_0: read-var descriptor:bv
        let s_1_0: Bits = fn_state.descriptor;
        // D s_1_1: size-of s_1_0
        let s_1_1: u16 = s_1_0.length();
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // C s_1_3: const #9s : i
        let s_1_3: i128 = 9;
        // D s_1_4: cmp-lt s_1_3 s_1_2
        let s_1_4: bool = ((s_1_3) < (s_1_2));
        // N s_1_5: assert s_1_4
        let s_1_5: () = assert!(s_1_4);
        // C s_1_6: const #8s : i
        let s_1_6: i128 = 8;
        // D s_1_7: read-var descriptor:bv
        let s_1_7: Bits = fn_state.descriptor;
        // C s_1_8: const #1s : i64
        let s_1_8: i64 = 1;
        // C s_1_9: cast zx s_1_8 -> i
        let s_1_9: i128 = (i128::try_from(s_1_8).unwrap());
        // C s_1_10: const #1s : i
        let s_1_10: i128 = 1;
        // C s_1_11: add s_1_10 s_1_9
        let s_1_11: i128 = (s_1_10 + s_1_9);
        // D s_1_12: bit-extract s_1_7 s_1_6 s_1_11
        let s_1_12: Bits = (Bits::new(
            ((s_1_7) >> (s_1_6)).value(),
            u16::try_from(s_1_11).unwrap(),
        ));
        // D s_1_13: cast reint s_1_12 -> u8
        let s_1_13: u8 = (s_1_12.value() as u8);
        // D s_1_14: write-var s2_sh <= s_1_13
        fn_state.s2_sh = s_1_13;
        // N s_1_15: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_2_0: const #11s : i
        let s_2_0: i128 = 11;
        // D s_2_1: read-var descriptor:bv
        let s_2_1: Bits = fn_state.descriptor;
        // C s_2_2: const #1u : u64
        let s_2_2: u64 = 1;
        // D s_2_3: bit-extract s_2_1 s_2_0 s_2_2
        let s_2_3: Bits = (Bits::new(
            ((s_2_1) >> (s_2_0)).value(),
            u16::try_from(s_2_2).unwrap(),
        ));
        // D s_2_4: cast reint s_2_3 -> u8
        let s_2_4: bool = ((s_2_3.value()) != 0);
        // C s_2_5: const #0s : i
        let s_2_5: i128 = 0;
        // C s_2_6: const #0u : u64
        let s_2_6: u64 = 0;
        // D s_2_7: cast zx s_2_4 -> u64
        let s_2_7: u64 = (s_2_4 as u64);
        // C s_2_8: const #1u : u64
        let s_2_8: u64 = 1;
        // D s_2_9: and s_2_7 s_2_8
        let s_2_9: u64 = ((s_2_7) & (s_2_8));
        // D s_2_10: cmp-eq s_2_9 s_2_8
        let s_2_10: bool = ((s_2_9) == (s_2_8));
        // D s_2_11: lsl s_2_7 s_2_5
        let s_2_11: u64 = s_2_7 << s_2_5;
        // D s_2_12: or s_2_6 s_2_11
        let s_2_12: u64 = ((s_2_6) | (s_2_11));
        // D s_2_13: cmpl s_2_11
        let s_2_13: u64 = !s_2_11;
        // D s_2_14: and s_2_6 s_2_13
        let s_2_14: u64 = ((s_2_6) & (s_2_13));
        // D s_2_15: select s_2_10 s_2_12 s_2_14
        let s_2_15: u64 = if s_2_10 { s_2_12 } else { s_2_14 };
        // D s_2_16: cast trunc s_2_15 -> u8
        let s_2_16: bool = ((s_2_15) != 0);
        // D s_2_17: write-var s2_fnxs <= s_2_16
        fn_state.s2_fnxs = s_2_16;
        // C s_2_18: const #2s : i
        let s_2_18: i128 = 2;
        // D s_2_19: read-var s2_attr:u8
        let s_2_19: u8 = fn_state.s2_attr;
        // D s_2_20: cast zx s_2_19 -> bv
        let s_2_20: Bits = Bits::new(s_2_19 as u128, 4u16);
        // C s_2_21: const #1u : u64
        let s_2_21: u64 = 1;
        // D s_2_22: bit-extract s_2_20 s_2_18 s_2_21
        let s_2_22: Bits = (Bits::new(
            ((s_2_20) >> (s_2_18)).value(),
            u16::try_from(s_2_21).unwrap(),
        ));
        // D s_2_23: cast reint s_2_22 -> u8
        let s_2_23: bool = ((s_2_22.value()) != 0);
        // C s_2_24: const #0s : i
        let s_2_24: i128 = 0;
        // C s_2_25: const #0u : u64
        let s_2_25: u64 = 0;
        // D s_2_26: cast zx s_2_23 -> u64
        let s_2_26: u64 = (s_2_23 as u64);
        // C s_2_27: const #1u : u64
        let s_2_27: u64 = 1;
        // D s_2_28: and s_2_26 s_2_27
        let s_2_28: u64 = ((s_2_26) & (s_2_27));
        // D s_2_29: cmp-eq s_2_28 s_2_27
        let s_2_29: bool = ((s_2_28) == (s_2_27));
        // D s_2_30: lsl s_2_26 s_2_24
        let s_2_30: u64 = s_2_26 << s_2_24;
        // D s_2_31: or s_2_25 s_2_30
        let s_2_31: u64 = ((s_2_25) | (s_2_30));
        // D s_2_32: cmpl s_2_30
        let s_2_32: u64 = !s_2_30;
        // D s_2_33: and s_2_25 s_2_32
        let s_2_33: u64 = ((s_2_25) & (s_2_32));
        // D s_2_34: select s_2_29 s_2_31 s_2_33
        let s_2_34: u64 = if s_2_29 { s_2_31 } else { s_2_33 };
        // D s_2_35: cast trunc s_2_34 -> u8
        let s_2_35: bool = ((s_2_34) != 0);
        // D s_2_36: cast zx s_2_35 -> bv
        let s_2_36: Bits = Bits::new(s_2_35 as u128, 1u16);
        // C s_2_37: const #0u : u8
        let s_2_37: bool = false;
        // C s_2_38: cast zx s_2_37 -> bv
        let s_2_38: Bits = Bits::new(s_2_37 as u128, 1u16);
        // D s_2_39: cmp-eq s_2_36 s_2_38
        let s_2_39: bool = ((s_2_36) == (s_2_38));
        // N s_2_40: branch s_2_39 b29 b3
        if s_2_39 {
            return block_29(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_3_0: const #0s : i
        let s_3_0: i128 = 0;
        // D s_3_1: read-var s2_attr:u8
        let s_3_1: u8 = fn_state.s2_attr;
        // D s_3_2: cast zx s_3_1 -> bv
        let s_3_2: Bits = Bits::new(s_3_1 as u128, 4u16);
        // C s_3_3: const #1s : i64
        let s_3_3: i64 = 1;
        // C s_3_4: cast zx s_3_3 -> i
        let s_3_4: i128 = (i128::try_from(s_3_3).unwrap());
        // C s_3_5: const #1s : i
        let s_3_5: i128 = 1;
        // C s_3_6: add s_3_5 s_3_4
        let s_3_6: i128 = (s_3_5 + s_3_4);
        // D s_3_7: bit-extract s_3_2 s_3_0 s_3_6
        let s_3_7: Bits = (Bits::new(
            ((s_3_2) >> (s_3_0)).value(),
            u16::try_from(s_3_6).unwrap(),
        ));
        // D s_3_8: cast reint s_3_7 -> u8
        let s_3_8: u8 = (s_3_7.value() as u8);
        // D s_3_9: cast zx s_3_8 -> bv
        let s_3_9: Bits = Bits::new(s_3_8 as u128, 2u16);
        // C s_3_10: const #3u : u8
        let s_3_10: u8 = 3;
        // C s_3_11: cast zx s_3_10 -> bv
        let s_3_11: Bits = Bits::new(s_3_10 as u128, 2u16);
        // D s_3_12: cmp-eq s_3_9 s_3_11
        let s_3_12: bool = ((s_3_9) == (s_3_11));
        // N s_3_13: branch s_3_12 b28 b4
        if s_3_12 {
            return block_28(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_4_0: const #0s : i
        let s_4_0: i128 = 0;
        // D s_4_1: read-var s2_attr:u8
        let s_4_1: u8 = fn_state.s2_attr;
        // D s_4_2: cast zx s_4_1 -> bv
        let s_4_2: Bits = Bits::new(s_4_1 as u128, 4u16);
        // C s_4_3: const #1s : i64
        let s_4_3: i64 = 1;
        // C s_4_4: cast zx s_4_3 -> i
        let s_4_4: i128 = (i128::try_from(s_4_3).unwrap());
        // C s_4_5: const #1s : i
        let s_4_5: i128 = 1;
        // C s_4_6: add s_4_5 s_4_4
        let s_4_6: i128 = (s_4_5 + s_4_4);
        // D s_4_7: bit-extract s_4_2 s_4_0 s_4_6
        let s_4_7: Bits = (Bits::new(
            ((s_4_2) >> (s_4_0)).value(),
            u16::try_from(s_4_6).unwrap(),
        ));
        // D s_4_8: cast reint s_4_7 -> u8
        let s_4_8: u8 = (s_4_7.value() as u8);
        // D s_4_9: cast zx s_4_8 -> bv
        let s_4_9: Bits = Bits::new(s_4_8 as u128, 2u16);
        // C s_4_10: const #2u : u8
        let s_4_10: u8 = 2;
        // C s_4_11: cast zx s_4_10 -> bv
        let s_4_11: Bits = Bits::new(s_4_10 as u128, 2u16);
        // D s_4_12: cmp-eq s_4_9 s_4_11
        let s_4_12: bool = ((s_4_9) == (s_4_11));
        // N s_4_13: branch s_4_12 b15 b5
        if s_4_12 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_5_0: read-var s1_memattrs.2:struct
        let s_5_0: u32 = fn_state.s1_memattrs._2;
        // C s_5_1: const #1u : u32
        let s_5_1: u32 = 1;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // N s_5_3: branch s_5_2 b14 b6
        if s_5_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_6_0: const #464u : u32
        let s_6_0: u32 = 464;
        // D s_6_1: read-reg s_6_0:u8
        let s_6_1: u8 = {
            let value = state.read_register::<u8>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: write-var cacheability_attr.0 <= s_6_1
        fn_state.cacheability_attr._0 = s_6_1;
        // C s_6_3: const #0u : u32
        let s_6_3: u32 = 0;
        // D s_6_4: write-var memattrs.2 <= s_6_3
        fn_state.memattrs._2 = s_6_3;
        // D s_6_5: read-var cacheability_attr:struct
        let s_6_5: ProductType183e6678e5239c85 = fn_state.cacheability_attr;
        // D s_6_6: write-var memattrs.1 <= s_6_5
        fn_state.memattrs._1 = s_6_5;
        // D s_6_7: read-var cacheability_attr:struct
        let s_6_7: ProductType183e6678e5239c85 = fn_state.cacheability_attr;
        // D s_6_8: write-var memattrs.4 <= s_6_7
        fn_state.memattrs._4 = s_6_7;
        // D s_6_9: read-var s1_memattrs.7:struct
        let s_6_9: bool = fn_state.s1_memattrs._7;
        // D s_6_10: write-var memattrs.7 <= s_6_9
        fn_state.memattrs._7 = s_6_9;
        // N s_6_11: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_7_0: read-var s2_sh:u8
        let s_7_0: u8 = fn_state.s2_sh;
        // D s_7_1: call DecodeShareability(s_7_0)
        let s_7_1: u32 = DecodeShareability(state, tracer, s_7_0);
        // D s_7_2: read-var s1_memattrs.5:struct
        let s_7_2: u32 = fn_state.s1_memattrs._5;
        // D s_7_3: call S2CombineS1Shareability(s_7_2, s_7_1)
        let s_7_3: u32 = S2CombineS1Shareability(state, tracer, s_7_2, s_7_1);
        // D s_7_4: write-var memattrs.5 <= s_7_3
        fn_state.memattrs._5 = s_7_3;
        // D s_7_5: read-var s1_memattrs.6:struct
        let s_7_5: u32 = fn_state.s1_memattrs._6;
        // D s_7_6: read-var memattrs:struct
        let s_7_6: ProductTypef170cab34335b70c = fn_state.memattrs;
        // D s_7_7: call S2MemTagType(s_7_6, s_7_5)
        let s_7_7: u32 = S2MemTagType(state, tracer, s_7_6, s_7_5);
        // D s_7_8: write-var memattrs.6 <= s_7_7
        fn_state.memattrs._6 = s_7_7;
        // C s_7_9: const #1s : i
        let s_7_9: i128 = 1;
        // D s_7_10: read-var s2_attr:u8
        let s_7_10: u8 = fn_state.s2_attr;
        // D s_7_11: cast zx s_7_10 -> bv
        let s_7_11: Bits = Bits::new(s_7_10 as u128, 4u16);
        // C s_7_12: const #1s : i64
        let s_7_12: i64 = 1;
        // C s_7_13: cast zx s_7_12 -> i
        let s_7_13: i128 = (i128::try_from(s_7_12).unwrap());
        // C s_7_14: const #2s : i
        let s_7_14: i128 = 2;
        // C s_7_15: add s_7_14 s_7_13
        let s_7_15: i128 = (s_7_14 + s_7_13);
        // D s_7_16: bit-extract s_7_11 s_7_9 s_7_15
        let s_7_16: Bits = (Bits::new(
            ((s_7_11) >> (s_7_9)).value(),
            u16::try_from(s_7_15).unwrap(),
        ));
        // D s_7_17: cast reint s_7_16 -> u8
        let s_7_17: u8 = (s_7_16.value() as u8);
        // D s_7_18: cast zx s_7_17 -> bv
        let s_7_18: Bits = Bits::new(s_7_17 as u128, 3u16);
        // C s_7_19: const #7u : u8
        let s_7_19: u8 = 7;
        // C s_7_20: cast zx s_7_19 -> bv
        let s_7_20: Bits = Bits::new(s_7_19 as u128, 3u16);
        // D s_7_21: cmp-eq s_7_18 s_7_20
        let s_7_21: bool = ((s_7_18) == (s_7_20));
        // N s_7_22: branch s_7_21 b13 b8
        if s_7_21 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#19070 <= s_8_0
        fn_state.gs_19070 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_9_0: read-var gs#19070:u8
        let s_9_0: bool = fn_state.gs_19070;
        // D s_9_1: write-var memattrs.3 <= s_9_0
        fn_state.memattrs._3 = s_9_0;
        // D s_9_2: read-var s2_fnxs:u8
        let s_9_2: bool = fn_state.s2_fnxs;
        // D s_9_3: cast zx s_9_2 -> bv
        let s_9_3: Bits = Bits::new(s_9_2 as u128, 1u16);
        // C s_9_4: const #1u : u8
        let s_9_4: bool = true;
        // C s_9_5: cast zx s_9_4 -> bv
        let s_9_5: Bits = Bits::new(s_9_4 as u128, 1u16);
        // D s_9_6: cmp-eq s_9_3 s_9_5
        let s_9_6: bool = ((s_9_3) == (s_9_5));
        // N s_9_7: branch s_9_6 b12 b10
        if s_9_6 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // N s_10_0: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_11_0: read-var memattrs:struct
        let s_11_0: ProductTypef170cab34335b70c = fn_state.memattrs;
        // D s_11_1: call EffectiveShareability(s_11_0)
        let s_11_1: u32 = EffectiveShareability(state, tracer, s_11_0);
        // D s_11_2: write-var memattrs.5 <= s_11_1
        fn_state.memattrs._5 = s_11_1;
        // D s_11_3: read-var memattrs:struct
        let s_11_3: ProductTypef170cab34335b70c = fn_state.memattrs;
        // N s_11_4: return s_11_3
        return s_11_3;
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_12_0: const #0u : u8
        let s_12_0: bool = false;
        // D s_12_1: write-var memattrs.7 <= s_12_0
        fn_state.memattrs._7 = s_12_0;
        // N s_12_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_13_0: read-var memattrs.6:struct
        let s_13_0: u32 = fn_state.memattrs._6;
        // C s_13_1: const #1u : u32
        let s_13_1: u32 = 1;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: write-var gs#19070 <= s_13_2
        fn_state.gs_19070 = s_13_2;
        // N s_13_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_14_0: read-var s1_memattrs:struct
        let s_14_0: ProductTypef170cab34335b70c = fn_state.s1_memattrs;
        // D s_14_1: write-var memattrs <= s_14_0
        fn_state.memattrs = s_14_0;
        // N s_14_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_15_0: const #0u : u32
        let s_15_0: u32 = 0;
        // D s_15_1: write-var memattrs.2 <= s_15_0
        fn_state.memattrs._2 = s_15_0;
        // C s_15_2: const #480u : u32
        let s_15_2: u32 = 480;
        // D s_15_3: read-reg s_15_2:u8
        let s_15_3: u8 = {
            let value = state.read_register::<u8>(s_15_2 as isize);
            tracer.read_register(s_15_2 as isize, value);
            value
        };
        // D s_15_4: write-var memattrs.1.0 <= s_15_3
        fn_state.memattrs._1._0 = s_15_3;
        // C s_15_5: const #480u : u32
        let s_15_5: u32 = 480;
        // D s_15_6: read-reg s_15_5:u8
        let s_15_6: u8 = {
            let value = state.read_register::<u8>(s_15_5 as isize);
            tracer.read_register(s_15_5 as isize, value);
            value
        };
        // D s_15_7: write-var memattrs.4.0 <= s_15_6
        fn_state.memattrs._4._0 = s_15_6;
        // D s_15_8: read-var s1_memattrs.2:struct
        let s_15_8: u32 = fn_state.s1_memattrs._2;
        // C s_15_9: const #0u : u32
        let s_15_9: u32 = 0;
        // D s_15_10: cmp-eq s_15_8 s_15_9
        let s_15_10: bool = ((s_15_8) == (s_15_9));
        // N s_15_11: branch s_15_10 b27 b16
        if s_15_10 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#19053 <= s_16_0
        fn_state.gs_19053 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_17_0: read-var gs#19053:u8
        let s_17_0: bool = fn_state.gs_19053;
        // N s_17_1: branch s_17_0 b26 b18
        if s_17_0 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_18_0: const #512u : u32
        let s_18_0: u32 = 512;
        // D s_18_1: read-reg s_18_0:u8
        let s_18_1: u8 = {
            let value = state.read_register::<u8>(s_18_0 as isize);
            tracer.read_register(s_18_0 as isize, value);
            value
        };
        // D s_18_2: write-var memattrs.1.1 <= s_18_1
        fn_state.memattrs._1._1 = s_18_1;
        // C s_18_3: const #0u : u8
        let s_18_3: bool = false;
        // D s_18_4: write-var memattrs.1.2 <= s_18_3
        fn_state.memattrs._1._2 = s_18_3;
        // N s_18_5: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_19_0: read-var s1_memattrs.2:struct
        let s_19_0: u32 = fn_state.s1_memattrs._2;
        // C s_19_1: const #0u : u32
        let s_19_1: u32 = 0;
        // D s_19_2: cmp-eq s_19_0 s_19_1
        let s_19_2: bool = ((s_19_0) == (s_19_1));
        // N s_19_3: branch s_19_2 b25 b20
        if s_19_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_20_0: const #0u : u8
        let s_20_0: bool = false;
        // D s_20_1: write-var gs#19054 <= s_20_0
        fn_state.gs_19054 = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_21_0: read-var gs#19054:u8
        let s_21_0: bool = fn_state.gs_19054;
        // N s_21_1: branch s_21_0 b24 b22
        if s_21_0 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_22_0: const #512u : u32
        let s_22_0: u32 = 512;
        // D s_22_1: read-reg s_22_0:u8
        let s_22_1: u8 = {
            let value = state.read_register::<u8>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: write-var memattrs.4.1 <= s_22_1
        fn_state.memattrs._4._1 = s_22_1;
        // C s_22_3: const #0u : u8
        let s_22_3: bool = false;
        // D s_22_4: write-var memattrs.4.2 <= s_22_3
        fn_state.memattrs._4._2 = s_22_3;
        // N s_22_5: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_23_0: const #0u : u8
        let s_23_0: bool = false;
        // D s_23_1: write-var memattrs.7 <= s_23_0
        fn_state.memattrs._7 = s_23_0;
        // N s_23_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_24_0: read-var s1_memattrs.4:struct
        let s_24_0: ProductType183e6678e5239c85 = fn_state.s1_memattrs._4;
        // D s_24_1: write-var ga#14256 <= s_24_0
        fn_state.ga_14256 = s_24_0;
        // D s_24_2: read-var ga#14256.1:struct
        let s_24_2: u8 = fn_state.ga_14256._1;
        // D s_24_3: write-var memattrs.4.1 <= s_24_2
        fn_state.memattrs._4._1 = s_24_2;
        // D s_24_4: read-var s1_memattrs.4:struct
        let s_24_4: ProductType183e6678e5239c85 = fn_state.s1_memattrs._4;
        // D s_24_5: write-var ga#14257 <= s_24_4
        fn_state.ga_14257 = s_24_4;
        // D s_24_6: read-var ga#14257.2:struct
        let s_24_6: bool = fn_state.ga_14257._2;
        // D s_24_7: write-var memattrs.4.2 <= s_24_6
        fn_state.memattrs._4._2 = s_24_6;
        // N s_24_8: jump b23
        return block_23(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_25_0: read-var s1_memattrs.4:struct
        let s_25_0: ProductType183e6678e5239c85 = fn_state.s1_memattrs._4;
        // D s_25_1: write-var ga#14252 <= s_25_0
        fn_state.ga_14252 = s_25_0;
        // D s_25_2: read-var ga#14252.0:struct
        let s_25_2: u8 = fn_state.ga_14252._0;
        // D s_25_3: cast zx s_25_2 -> bv
        let s_25_3: Bits = Bits::new(s_25_2 as u128, 2u16);
        // C s_25_4: const #464u : u32
        let s_25_4: u32 = 464;
        // D s_25_5: read-reg s_25_4:u8
        let s_25_5: u8 = {
            let value = state.read_register::<u8>(s_25_4 as isize);
            tracer.read_register(s_25_4 as isize, value);
            value
        };
        // D s_25_6: cast zx s_25_5 -> bv
        let s_25_6: Bits = Bits::new(s_25_5 as u128, 2u16);
        // D s_25_7: cmp-ne s_25_3 s_25_6
        let s_25_7: bool = ((s_25_3) != (s_25_6));
        // D s_25_8: write-var gs#19054 <= s_25_7
        fn_state.gs_19054 = s_25_7;
        // N s_25_9: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_26_0: read-var s1_memattrs.1:struct
        let s_26_0: ProductType183e6678e5239c85 = fn_state.s1_memattrs._1;
        // D s_26_1: write-var ga#14249 <= s_26_0
        fn_state.ga_14249 = s_26_0;
        // D s_26_2: read-var ga#14249.1:struct
        let s_26_2: u8 = fn_state.ga_14249._1;
        // D s_26_3: write-var memattrs.1.1 <= s_26_2
        fn_state.memattrs._1._1 = s_26_2;
        // D s_26_4: read-var s1_memattrs.1:struct
        let s_26_4: ProductType183e6678e5239c85 = fn_state.s1_memattrs._1;
        // D s_26_5: write-var ga#14250 <= s_26_4
        fn_state.ga_14250 = s_26_4;
        // D s_26_6: read-var ga#14250.2:struct
        let s_26_6: bool = fn_state.ga_14250._2;
        // D s_26_7: write-var memattrs.1.2 <= s_26_6
        fn_state.memattrs._1._2 = s_26_6;
        // N s_26_8: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_27_0: read-var s1_memattrs.1:struct
        let s_27_0: ProductType183e6678e5239c85 = fn_state.s1_memattrs._1;
        // D s_27_1: write-var ga#14245 <= s_27_0
        fn_state.ga_14245 = s_27_0;
        // D s_27_2: read-var ga#14245.0:struct
        let s_27_2: u8 = fn_state.ga_14245._0;
        // D s_27_3: cast zx s_27_2 -> bv
        let s_27_3: Bits = Bits::new(s_27_2 as u128, 2u16);
        // C s_27_4: const #464u : u32
        let s_27_4: u32 = 464;
        // D s_27_5: read-reg s_27_4:u8
        let s_27_5: u8 = {
            let value = state.read_register::<u8>(s_27_4 as isize);
            tracer.read_register(s_27_4 as isize, value);
            value
        };
        // D s_27_6: cast zx s_27_5 -> bv
        let s_27_6: Bits = Bits::new(s_27_5 as u128, 2u16);
        // D s_27_7: cmp-ne s_27_3 s_27_6
        let s_27_7: bool = ((s_27_3) != (s_27_6));
        // D s_27_8: write-var gs#19053 <= s_27_7
        fn_state.gs_19053 = s_27_7;
        // N s_27_9: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_28_0: read-var s1_memattrs:struct
        let s_28_0: ProductTypef170cab34335b70c = fn_state.s1_memattrs;
        // D s_28_1: write-var memattrs <= s_28_0
        fn_state.memattrs = s_28_0;
        // N s_28_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // C s_29_0: const #0s : i
        let s_29_0: i128 = 0;
        // D s_29_1: read-var s2_attr:u8
        let s_29_1: u8 = fn_state.s2_attr;
        // D s_29_2: cast zx s_29_1 -> bv
        let s_29_2: Bits = Bits::new(s_29_1 as u128, 4u16);
        // C s_29_3: const #1s : i64
        let s_29_3: i64 = 1;
        // C s_29_4: cast zx s_29_3 -> i
        let s_29_4: i128 = (i128::try_from(s_29_3).unwrap());
        // C s_29_5: const #1s : i
        let s_29_5: i128 = 1;
        // C s_29_6: add s_29_5 s_29_4
        let s_29_6: i128 = (s_29_5 + s_29_4);
        // D s_29_7: bit-extract s_29_2 s_29_0 s_29_6
        let s_29_7: Bits = (Bits::new(
            ((s_29_2) >> (s_29_0)).value(),
            u16::try_from(s_29_6).unwrap(),
        ));
        // D s_29_8: cast reint s_29_7 -> u8
        let s_29_8: u8 = (s_29_7.value() as u8);
        // D s_29_9: call DecodeDevice(s_29_8)
        let s_29_9: u32 = DecodeDevice(state, tracer, s_29_8);
        // D s_29_10: write-var s2_deviceshadow#314 <= s_29_9
        fn_state.s2_deviceshadow_314 = s_29_9;
        // C s_29_11: const #1u : u32
        let s_29_11: u32 = 1;
        // D s_29_12: write-var memattrs.2 <= s_29_11
        fn_state.memattrs._2 = s_29_11;
        // D s_29_13: read-var s1_memattrs.2:struct
        let s_29_13: u32 = fn_state.s1_memattrs._2;
        // C s_29_14: const #1u : u32
        let s_29_14: u32 = 1;
        // D s_29_15: cmp-eq s_29_13 s_29_14
        let s_29_15: bool = ((s_29_13) == (s_29_14));
        // N s_29_16: branch s_29_15 b32 b30
        if s_29_15 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_30_0: read-var s2_deviceshadow#314:u32
        let s_30_0: u32 = fn_state.s2_deviceshadow_314;
        // D s_30_1: write-var memattrs.0 <= s_30_0
        fn_state.memattrs._0 = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_31_0: read-var s1_memattrs.7:struct
        let s_31_0: bool = fn_state.s1_memattrs._7;
        // D s_31_1: write-var memattrs.7 <= s_31_0
        fn_state.memattrs._7 = s_31_0;
        // N s_31_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_32_0: read-var s1_memattrs.0:struct
        let s_32_0: u32 = fn_state.s1_memattrs._0;
        // D s_32_1: read-var s2_deviceshadow#314:u32
        let s_32_1: u32 = fn_state.s2_deviceshadow_314;
        // D s_32_2: call S2CombineS1Device(s_32_0, s_32_1)
        let s_32_2: u32 = S2CombineS1Device(state, tracer, s_32_0, s_32_1);
        // D s_32_3: write-var memattrs.0 <= s_32_2
        fn_state.memattrs._0 = s_32_2;
        // N s_32_4: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef170cab34335b70c {
        // D s_33_0: read-var walkparams.20:struct
        let s_33_0: u8 = fn_state.walkparams._20;
        // D s_33_1: write-var s2_sh <= s_33_0
        fn_state.s2_sh = s_33_0;
        // N s_33_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
