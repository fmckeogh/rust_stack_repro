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
use neq_int::*;
use HavePMUv3EDGE::*;
use u__id::*;
use HavePMUv3TH::*;
use u_get_PMEVTYPER_EL0_Type_TH::*;
use HaveAArch64::*;
use ConstrainUnpredictableBits::*;
use u_get_PMEVTYPER_EL0_Type_TE::*;
use u_get_PMEVTYPER_EL0_Type_TC::*;
use common::*;
pub fn PMUCountValue<T: Tracer>(
    state: &mut State,
    tracer: &T,
    n: i64,
    Vb: i128,
) -> i128 {
    #[derive(Default)]
    struct FunctionState {
        gs_2758: bool,
        ga_1616: u8,
        Vc: bool,
        v: i128,
        Vt: i128,
        gs_431988: ProductType690b94b58c91cec7,
        Vpshadow_29: bool,
        Vtshadow_27: i128,
        return_value: i128,
        tc: u8,
        T: i64,
        gs_2775: bool,
        gs_2754: bool,
        gs_2735: bool,
        n: i64,
        Vb: i128,
    }
    let fn_state = FunctionState {
        n,
        Vb,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_0_0: const #() : ()
        let s_0_0: () = ();
        // S s_0_1: call HavePMUv3TH(s_0_0)
        let s_0_1: bool = HavePMUv3TH(state, tracer, s_0_0);
        // S s_0_2: not s_0_1
        let s_0_2: bool = !s_0_1;
        // N s_0_3: branch s_0_2 b53 b1
        if s_0_2 {
            return block_53(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_1_0: const #() : ()
        let s_1_0: () = ();
        // S s_1_1: call HaveAArch64(s_1_0)
        let s_1_1: bool = HaveAArch64(state, tracer, s_1_0);
        // S s_1_2: not s_1_1
        let s_1_2: bool = !s_1_1;
        // D s_1_3: write-var gs#2735 <= s_1_2
        fn_state.gs_2735 = s_1_2;
        // N s_1_4: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_2_0: read-var gs#2735:u8
        let s_2_0: bool = fn_state.gs_2735;
        // N s_2_1: branch s_2_0 b52 b3
        if s_2_0 {
            return block_52(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_3_0: const #11208u : u32
        let s_3_0: u32 = 11208;
        // D s_3_1: read-reg s_3_0:[struct; 32]
        let s_3_1: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 32usize]>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: read-var n:i64
        let s_3_2: i64 = fn_state.n;
        // D s_3_3: cast zx s_3_2 -> i
        let s_3_3: i128 = (i128::try_from(s_3_2).unwrap());
        // D s_3_4: read-element s_3_1[s_3_3]
        let s_3_4: ProductType5c790c8ef59cc8b2 = s_3_1[(s_3_3) as usize];
        // D s_3_5: call _get_PMEVTYPER_EL0_Type_TH(s_3_4)
        let s_3_5: u16 = u_get_PMEVTYPER_EL0_Type_TH(state, tracer, s_3_4);
        // D s_3_6: cast zx s_3_5 -> bv
        let s_3_6: Bits = Bits::new(s_3_5 as u128, 12u16);
        // D s_3_7: cast zx s_3_6 -> i
        let s_3_7: i128 = (s_3_6.value() as i128);
        // D s_3_8: cast reint s_3_7 -> i64
        let s_3_8: i64 = (s_3_7 as i64);
        // D s_3_9: write-var T <= s_3_8
        fn_state.T = s_3_8;
        // C s_3_10: const #11208u : u32
        let s_3_10: u32 = 11208;
        // D s_3_11: read-reg s_3_10:[struct; 32]
        let s_3_11: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_3_10 as isize);
            tracer.read_register(s_3_10 as isize, value);
            value
        };
        // D s_3_12: read-var n:i64
        let s_3_12: i64 = fn_state.n;
        // D s_3_13: cast zx s_3_12 -> i
        let s_3_13: i128 = (i128::try_from(s_3_12).unwrap());
        // D s_3_14: read-element s_3_11[s_3_13]
        let s_3_14: ProductType5c790c8ef59cc8b2 = s_3_11[(s_3_13) as usize];
        // D s_3_15: call _get_PMEVTYPER_EL0_Type_TC(s_3_14)
        let s_3_15: u8 = u_get_PMEVTYPER_EL0_Type_TC(state, tracer, s_3_14);
        // C s_3_16: const #1s : i
        let s_3_16: i128 = 1;
        // D s_3_17: cast zx s_3_15 -> bv
        let s_3_17: Bits = Bits::new(s_3_15 as u128, 3u16);
        // C s_3_18: const #1s : i64
        let s_3_18: i64 = 1;
        // C s_3_19: cast zx s_3_18 -> i
        let s_3_19: i128 = (i128::try_from(s_3_18).unwrap());
        // C s_3_20: const #1s : i
        let s_3_20: i128 = 1;
        // C s_3_21: add s_3_20 s_3_19
        let s_3_21: i128 = (s_3_20 + s_3_19);
        // D s_3_22: bit-extract s_3_17 s_3_16 s_3_21
        let s_3_22: Bits = (Bits::new(
            ((s_3_17) >> (s_3_16)).value(),
            u16::try_from(s_3_21).unwrap(),
        ));
        // D s_3_23: cast reint s_3_22 -> u8
        let s_3_23: u8 = (s_3_22.value() as u8);
        // D s_3_24: write-var ga#1616 <= s_3_23
        fn_state.ga_1616 = s_3_23;
        // D s_3_25: read-var ga#1616:u8
        let s_3_25: u8 = fn_state.ga_1616;
        // D s_3_26: cast zx s_3_25 -> bv
        let s_3_26: Bits = Bits::new(s_3_25 as u128, 2u16);
        // C s_3_27: const #0u : u8
        let s_3_27: u8 = 0;
        // C s_3_28: cast zx s_3_27 -> bv
        let s_3_28: Bits = Bits::new(s_3_27 as u128, 2u16);
        // D s_3_29: cmp-eq s_3_26 s_3_28
        let s_3_29: bool = ((s_3_26) == (s_3_28));
        // D s_3_30: not s_3_29
        let s_3_30: bool = !s_3_29;
        // N s_3_31: branch s_3_30 b45 b4
        if s_3_30 {
            return block_45(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_4_0: read-var T:i64
        let s_4_0: i64 = fn_state.T;
        // D s_4_1: cast zx s_4_0 -> i
        let s_4_1: i128 = (i128::try_from(s_4_0).unwrap());
        // D s_4_2: read-var Vb:i
        let s_4_2: i128 = fn_state.Vb;
        // D s_4_3: call neq_int(s_4_2, s_4_1)
        let s_4_3: bool = neq_int(state, tracer, s_4_2, s_4_1);
        // D s_4_4: write-var Vc <= s_4_3
        fn_state.Vc = s_4_3;
        // N s_4_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_5_0: const #11208u : u32
        let s_5_0: u32 = 11208;
        // D s_5_1: read-reg s_5_0:[struct; 32]
        let s_5_1: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<[ProductType5c790c8ef59cc8b2; 32usize]>(s_5_0 as isize);
            tracer.read_register(s_5_0 as isize, value);
            value
        };
        // D s_5_2: read-var n:i64
        let s_5_2: i64 = fn_state.n;
        // D s_5_3: cast zx s_5_2 -> i
        let s_5_3: i128 = (i128::try_from(s_5_2).unwrap());
        // D s_5_4: read-element s_5_1[s_5_3]
        let s_5_4: ProductType5c790c8ef59cc8b2 = s_5_1[(s_5_3) as usize];
        // D s_5_5: call _get_PMEVTYPER_EL0_Type_TC(s_5_4)
        let s_5_5: u8 = u_get_PMEVTYPER_EL0_Type_TC(state, tracer, s_5_4);
        // C s_5_6: const #0s : i
        let s_5_6: i128 = 0;
        // D s_5_7: cast zx s_5_5 -> bv
        let s_5_7: Bits = Bits::new(s_5_5 as u128, 3u16);
        // C s_5_8: const #1u : u64
        let s_5_8: u64 = 1;
        // D s_5_9: bit-extract s_5_7 s_5_6 s_5_8
        let s_5_9: Bits = (Bits::new(
            ((s_5_7) >> (s_5_6)).value(),
            u16::try_from(s_5_8).unwrap(),
        ));
        // D s_5_10: cast reint s_5_9 -> u8
        let s_5_10: bool = ((s_5_9.value()) != 0);
        // C s_5_11: const #0s : i
        let s_5_11: i128 = 0;
        // C s_5_12: const #0u : u64
        let s_5_12: u64 = 0;
        // D s_5_13: cast zx s_5_10 -> u64
        let s_5_13: u64 = (s_5_10 as u64);
        // C s_5_14: const #1u : u64
        let s_5_14: u64 = 1;
        // D s_5_15: and s_5_13 s_5_14
        let s_5_15: u64 = ((s_5_13) & (s_5_14));
        // D s_5_16: cmp-eq s_5_15 s_5_14
        let s_5_16: bool = ((s_5_15) == (s_5_14));
        // D s_5_17: lsl s_5_13 s_5_11
        let s_5_17: u64 = s_5_13 << s_5_11;
        // D s_5_18: or s_5_12 s_5_17
        let s_5_18: u64 = ((s_5_12) | (s_5_17));
        // D s_5_19: cmpl s_5_17
        let s_5_19: u64 = !s_5_17;
        // D s_5_20: and s_5_12 s_5_19
        let s_5_20: u64 = ((s_5_12) & (s_5_19));
        // D s_5_21: select s_5_16 s_5_18 s_5_20
        let s_5_21: u64 = if s_5_16 { s_5_18 } else { s_5_20 };
        // D s_5_22: cast trunc s_5_21 -> u8
        let s_5_22: bool = ((s_5_21) != 0);
        // D s_5_23: cast zx s_5_22 -> bv
        let s_5_23: Bits = Bits::new(s_5_22 as u128, 1u16);
        // C s_5_24: const #0u : u8
        let s_5_24: bool = false;
        // C s_5_25: cast zx s_5_24 -> bv
        let s_5_25: Bits = Bits::new(s_5_24 as u128, 1u16);
        // D s_5_26: cmp-eq s_5_23 s_5_25
        let s_5_26: bool = ((s_5_23) == (s_5_25));
        // N s_5_27: branch s_5_26 b41 b6
        if s_5_26 {
            return block_41(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_6_0: read-var Vc:u8
        let s_6_0: bool = fn_state.Vc;
        // N s_6_1: branch s_6_0 b40 b7
        if s_6_0 {
            return block_40(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_7_0: const #0s : i
        let s_7_0: i128 = 0;
        // D s_7_1: write-var Vt <= s_7_0
        fn_state.Vt = s_7_0;
        // N s_7_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_8_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_9_0: read-var Vt:i
        let s_9_0: i128 = fn_state.Vt;
        // D s_9_1: write-var Vtshadow#27 <= s_9_0
        fn_state.Vtshadow_27 = s_9_0;
        // C s_9_2: const #() : ()
        let s_9_2: () = ();
        // S s_9_3: call HavePMUv3EDGE(s_9_2)
        let s_9_3: bool = HavePMUv3EDGE(state, tracer, s_9_2);
        // N s_9_4: branch s_9_3 b39 b10
        if s_9_3 {
            return block_39(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_10_0: const #0u : u8
        let s_10_0: bool = false;
        // D s_10_1: write-var gs#2754 <= s_10_0
        fn_state.gs_2754 = s_10_0;
        // N s_10_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_11_0: read-var gs#2754:u8
        let s_11_0: bool = fn_state.gs_2754;
        // N s_11_1: branch s_11_0 b15 b12
        if s_11_0 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_12_0: read-var Vtshadow#27:i
        let s_12_0: i128 = fn_state.Vtshadow_27;
        // D s_12_1: write-var v <= s_12_0
        fn_state.v = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_13_0: read-var v:i
        let s_13_0: i128 = fn_state.v;
        // C s_13_1: const #10560u : u32
        let s_13_1: u32 = 10560;
        // D s_13_2: read-reg s_13_1:[u8; 31]
        let s_13_2: [bool; 31usize] = {
            let value = state.read_register::<[bool; 31usize]>(s_13_1 as isize);
            tracer.read_register(s_13_1 as isize, value);
            value
        };
        // D s_13_3: read-var n:i64
        let s_13_3: i64 = fn_state.n;
        // D s_13_4: cast zx s_13_3 -> i
        let s_13_4: i128 = (i128::try_from(s_13_3).unwrap());
        // D s_13_5: read-var Vc:u8
        let s_13_5: bool = fn_state.Vc;
        // D s_13_6: mutate-element s_13_2[s_13_4] <= s_13_5
        let s_13_6: [bool; 31usize] = {
            let mut local = s_13_2.clone();
            local[(s_13_4) as usize] = s_13_5;
            local
        };
        // D s_13_7: cast cvt s_13_6 -> [u8; 0]
        let s_13_7: alloc::vec::Vec<bool> = alloc::vec::Vec::from(s_13_6);
        // D s_13_8: cast cvt s_13_7 -> [u8; 31]
        let s_13_8: [bool; 31usize] = {
            let mut buf = [Default::default(); 31usize];
            buf.copy_from_slice(&s_13_7);
            buf
        };
        // C s_13_9: const #10560u : u32
        let s_13_9: u32 = 10560;
        // N s_13_10: write-reg s_13_9 <= s_13_8
        let s_13_10: () = {
            state.write_register::<[bool; 31usize]>(s_13_9 as isize, s_13_8);
            tracer.write_register(s_13_9 as isize, s_13_8);
        };
        // D s_13_11: write-var return_value <= s_13_0
        fn_state.return_value = s_13_0;
        // N s_13_12: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_14_0: read-var return_value:i
        let s_14_0: i128 = fn_state.return_value;
        // N s_14_1: return s_14_0
        return s_14_0;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_15_0: read-var n:i64
        let s_15_0: i64 = fn_state.n;
        // D s_15_1: cast zx s_15_0 -> i
        let s_15_1: i128 = (i128::try_from(s_15_0).unwrap());
        // D s_15_2: call __id(s_15_1)
        let s_15_2: i128 = u__id(state, tracer, s_15_1);
        // D s_15_3: cast reint s_15_2 -> i64
        let s_15_3: i64 = (s_15_2 as i64);
        // C s_15_4: const #0s : i
        let s_15_4: i128 = 0;
        // D s_15_5: cast zx s_15_3 -> i
        let s_15_5: i128 = (i128::try_from(s_15_3).unwrap());
        // D s_15_6: cmp-le s_15_4 s_15_5
        let s_15_6: bool = ((s_15_4) <= (s_15_5));
        // N s_15_7: branch s_15_6 b38 b16
        if s_15_6 {
            return block_38(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var gs#2758 <= s_16_0
        fn_state.gs_2758 = s_16_0;
        // N s_16_2: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_17_0: read-var gs#2758:u8
        let s_17_0: bool = fn_state.gs_2758;
        // N s_17_1: assert s_17_0
        let s_17_1: () = assert!(s_17_0);
        // C s_17_2: const #10560u : u32
        let s_17_2: u32 = 10560;
        // D s_17_3: read-reg s_17_2:[u8; 31]
        let s_17_3: [bool; 31usize] = {
            let value = state.read_register::<[bool; 31usize]>(s_17_2 as isize);
            tracer.read_register(s_17_2 as isize, value);
            value
        };
        // D s_17_4: read-var n:i64
        let s_17_4: i64 = fn_state.n;
        // D s_17_5: cast zx s_17_4 -> i
        let s_17_5: i128 = (i128::try_from(s_17_4).unwrap());
        // D s_17_6: read-element s_17_3[s_17_5]
        let s_17_6: bool = s_17_3[(s_17_5) as usize];
        // D s_17_7: write-var Vpshadow#29 <= s_17_6
        fn_state.Vpshadow_29 = s_17_6;
        // C s_17_8: const #11208u : u32
        let s_17_8: u32 = 11208;
        // D s_17_9: read-reg s_17_8:[struct; 32]
        let s_17_9: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_17_8 as isize);
            tracer.read_register(s_17_8 as isize, value);
            value
        };
        // D s_17_10: read-var n:i64
        let s_17_10: i64 = fn_state.n;
        // D s_17_11: cast zx s_17_10 -> i
        let s_17_11: i128 = (i128::try_from(s_17_10).unwrap());
        // D s_17_12: read-element s_17_9[s_17_11]
        let s_17_12: ProductType5c790c8ef59cc8b2 = s_17_9[(s_17_11) as usize];
        // D s_17_13: call _get_PMEVTYPER_EL0_Type_TC(s_17_12)
        let s_17_13: u8 = u_get_PMEVTYPER_EL0_Type_TC(state, tracer, s_17_12);
        // C s_17_14: const #0s : i
        let s_17_14: i128 = 0;
        // D s_17_15: cast zx s_17_13 -> bv
        let s_17_15: Bits = Bits::new(s_17_13 as u128, 3u16);
        // C s_17_16: const #1s : i64
        let s_17_16: i64 = 1;
        // C s_17_17: cast zx s_17_16 -> i
        let s_17_17: i128 = (i128::try_from(s_17_16).unwrap());
        // C s_17_18: const #1s : i
        let s_17_18: i128 = 1;
        // C s_17_19: add s_17_18 s_17_17
        let s_17_19: i128 = (s_17_18 + s_17_17);
        // D s_17_20: bit-extract s_17_15 s_17_14 s_17_19
        let s_17_20: Bits = (Bits::new(
            ((s_17_15) >> (s_17_14)).value(),
            u16::try_from(s_17_19).unwrap(),
        ));
        // D s_17_21: cast reint s_17_20 -> u8
        let s_17_21: u8 = (s_17_20.value() as u8);
        // D s_17_22: write-var tc <= s_17_21
        fn_state.tc = s_17_21;
        // D s_17_23: read-var tc:u8
        let s_17_23: u8 = fn_state.tc;
        // D s_17_24: cast zx s_17_23 -> bv
        let s_17_24: Bits = Bits::new(s_17_23 as u128, 2u16);
        // C s_17_25: const #0u : u8
        let s_17_25: u8 = 0;
        // C s_17_26: cast zx s_17_25 -> bv
        let s_17_26: Bits = Bits::new(s_17_25 as u128, 2u16);
        // D s_17_27: cmp-eq s_17_24 s_17_26
        let s_17_27: bool = ((s_17_24) == (s_17_26));
        // N s_17_28: branch s_17_27 b34 b18
        if s_17_27 {
            return block_34(state, tracer, fn_state);
        } else {
            return block_18(state, tracer, fn_state);
        };
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_18_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_19_0: read-var tc:u8
        let s_19_0: u8 = fn_state.tc;
        // D s_19_1: cast zx s_19_0 -> bv
        let s_19_1: Bits = Bits::new(s_19_0 as u128, 2u16);
        // C s_19_2: const #0u : u8
        let s_19_2: u8 = 0;
        // C s_19_3: cast zx s_19_2 -> bv
        let s_19_3: Bits = Bits::new(s_19_2 as u128, 2u16);
        // D s_19_4: cmp-eq s_19_1 s_19_3
        let s_19_4: bool = ((s_19_1) == (s_19_3));
        // D s_19_5: not s_19_4
        let s_19_5: bool = !s_19_4;
        // N s_19_6: branch s_19_5 b22 b20
        if s_19_5 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_20(state, tracer, fn_state);
        };
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_20_0: read-var Vtshadow#27:i
        let s_20_0: i128 = fn_state.Vtshadow_27;
        // D s_20_1: write-var v <= s_20_0
        fn_state.v = s_20_0;
        // N s_20_2: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_21_0: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_22_0: read-var tc:u8
        let s_22_0: u8 = fn_state.tc;
        // D s_22_1: cast zx s_22_0 -> bv
        let s_22_1: Bits = Bits::new(s_22_0 as u128, 2u16);
        // C s_22_2: const #2u : u8
        let s_22_2: u8 = 2;
        // C s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 2u16);
        // D s_22_4: cmp-eq s_22_1 s_22_3
        let s_22_4: bool = ((s_22_1) == (s_22_3));
        // D s_22_5: not s_22_4
        let s_22_5: bool = !s_22_4;
        // N s_22_6: branch s_22_5 b27 b23
        if s_22_5 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_23(state, tracer, fn_state);
        };
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_23_0: read-var Vpshadow#29:u8
        let s_23_0: bool = fn_state.Vpshadow_29;
        // D s_23_1: read-var Vc:u8
        let s_23_1: bool = fn_state.Vc;
        // D s_23_2: cmp-ne s_23_0 s_23_1
        let s_23_2: bool = ((s_23_0) != (s_23_1));
        // N s_23_3: branch s_23_2 b26 b24
        if s_23_2 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_24(state, tracer, fn_state);
        };
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_24_0: const #0s : i
        let s_24_0: i128 = 0;
        // D s_24_1: write-var v <= s_24_0
        fn_state.v = s_24_0;
        // N s_24_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_25_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_26_0: const #1s : i
        let s_26_0: i128 = 1;
        // D s_26_1: write-var v <= s_26_0
        fn_state.v = s_26_0;
        // N s_26_2: jump b25
        return block_25(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_27_0: read-var Vpshadow#29:u8
        let s_27_0: bool = fn_state.Vpshadow_29;
        // D s_27_1: not s_27_0
        let s_27_1: bool = !s_27_0;
        // N s_27_2: branch s_27_1 b33 b28
        if s_27_1 {
            return block_33(state, tracer, fn_state);
        } else {
            return block_28(state, tracer, fn_state);
        };
    }
    fn block_28<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_28_0: const #0u : u8
        let s_28_0: bool = false;
        // D s_28_1: write-var gs#2775 <= s_28_0
        fn_state.gs_2775 = s_28_0;
        // N s_28_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_29<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_29_0: read-var gs#2775:u8
        let s_29_0: bool = fn_state.gs_2775;
        // N s_29_1: branch s_29_0 b32 b30
        if s_29_0 {
            return block_32(state, tracer, fn_state);
        } else {
            return block_30(state, tracer, fn_state);
        };
    }
    fn block_30<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_30_0: const #0s : i
        let s_30_0: i128 = 0;
        // D s_30_1: write-var v <= s_30_0
        fn_state.v = s_30_0;
        // N s_30_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_31<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_31_0: jump b21
        return block_21(state, tracer, fn_state);
    }
    fn block_32<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_32_0: const #1s : i
        let s_32_0: i128 = 1;
        // D s_32_1: write-var v <= s_32_0
        fn_state.v = s_32_0;
        // N s_32_2: jump b31
        return block_31(state, tracer, fn_state);
    }
    fn block_33<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_33_0: read-var Vc:u8
        let s_33_0: bool = fn_state.Vc;
        // D s_33_1: write-var gs#2775 <= s_33_0
        fn_state.gs_2775 = s_33_0;
        // N s_33_2: jump b29
        return block_29(state, tracer, fn_state);
    }
    fn block_34<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_34_0: const #2s : i
        let s_34_0: i128 = 2;
        // C s_34_1: const #78u : u32
        let s_34_1: u32 = 78;
        // S s_34_2: call ConstrainUnpredictableBits(s_34_1, s_34_0)
        let s_34_2: ProductType690b94b58c91cec7 = ConstrainUnpredictableBits(
            state,
            tracer,
            s_34_1,
            s_34_0,
        );
        // D s_34_3: write-var gs#431988 <= s_34_2
        fn_state.gs_431988 = s_34_2;
        // D s_34_4: read-var gs#431988.0:struct
        let s_34_4: u32 = fn_state.gs_431988._0;
        // D s_34_5: read-var gs#431988.1:struct
        let s_34_5: Bits = fn_state.gs_431988._1;
        // D s_34_6: cast reint s_34_5 -> u8
        let s_34_6: u8 = (s_34_5.value() as u8);
        // D s_34_7: write-var tc <= s_34_6
        fn_state.tc = s_34_6;
        // C s_34_8: const #7u : u32
        let s_34_8: u32 = 7;
        // D s_34_9: cmp-eq s_34_4 s_34_8
        let s_34_9: bool = ((s_34_4) == (s_34_8));
        // N s_34_10: branch s_34_9 b37 b35
        if s_34_9 {
            return block_37(state, tracer, fn_state);
        } else {
            return block_35(state, tracer, fn_state);
        };
    }
    fn block_35<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_35_0: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_36<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_36_0: jump b19
        return block_19(state, tracer, fn_state);
    }
    fn block_37<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_37_0: const #0u : u8
        let s_37_0: u8 = 0;
        // D s_37_1: write-var tc <= s_37_0
        fn_state.tc = s_37_0;
        // N s_37_2: jump b36
        return block_36(state, tracer, fn_state);
    }
    fn block_38<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_38_0: read-var n:i64
        let s_38_0: i64 = fn_state.n;
        // D s_38_1: cast zx s_38_0 -> i
        let s_38_1: i128 = (i128::try_from(s_38_0).unwrap());
        // D s_38_2: call __id(s_38_1)
        let s_38_2: i128 = u__id(state, tracer, s_38_1);
        // D s_38_3: cast reint s_38_2 -> i64
        let s_38_3: i64 = (s_38_2 as i64);
        // C s_38_4: const #31s : i
        let s_38_4: i128 = 31;
        // D s_38_5: cast zx s_38_3 -> i
        let s_38_5: i128 = (i128::try_from(s_38_3).unwrap());
        // D s_38_6: cmp-lt s_38_5 s_38_4
        let s_38_6: bool = ((s_38_5) < (s_38_4));
        // D s_38_7: write-var gs#2758 <= s_38_6
        fn_state.gs_2758 = s_38_6;
        // N s_38_8: jump b17
        return block_17(state, tracer, fn_state);
    }
    fn block_39<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_39_0: const #11208u : u32
        let s_39_0: u32 = 11208;
        // D s_39_1: read-reg s_39_0:[struct; 32]
        let s_39_1: [ProductType5c790c8ef59cc8b2; 32usize] = {
            let value = state
                .read_register::<
                    [ProductType5c790c8ef59cc8b2; 32usize],
                >(s_39_0 as isize);
            tracer.read_register(s_39_0 as isize, value);
            value
        };
        // D s_39_2: read-var n:i64
        let s_39_2: i64 = fn_state.n;
        // D s_39_3: cast zx s_39_2 -> i
        let s_39_3: i128 = (i128::try_from(s_39_2).unwrap());
        // D s_39_4: read-element s_39_1[s_39_3]
        let s_39_4: ProductType5c790c8ef59cc8b2 = s_39_1[(s_39_3) as usize];
        // D s_39_5: call _get_PMEVTYPER_EL0_Type_TE(s_39_4)
        let s_39_5: bool = u_get_PMEVTYPER_EL0_Type_TE(state, tracer, s_39_4);
        // D s_39_6: cast zx s_39_5 -> bv
        let s_39_6: Bits = Bits::new(s_39_5 as u128, 1u16);
        // C s_39_7: const #1u : u8
        let s_39_7: bool = true;
        // C s_39_8: cast zx s_39_7 -> bv
        let s_39_8: Bits = Bits::new(s_39_7 as u128, 1u16);
        // D s_39_9: cmp-eq s_39_6 s_39_8
        let s_39_9: bool = ((s_39_6) == (s_39_8));
        // D s_39_10: write-var gs#2754 <= s_39_9
        fn_state.gs_2754 = s_39_9;
        // N s_39_11: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_40<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_40_0: const #1s : i
        let s_40_0: i128 = 1;
        // D s_40_1: write-var Vt <= s_40_0
        fn_state.Vt = s_40_0;
        // N s_40_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_41<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_41_0: read-var Vc:u8
        let s_41_0: bool = fn_state.Vc;
        // N s_41_1: branch s_41_0 b44 b42
        if s_41_0 {
            return block_44(state, tracer, fn_state);
        } else {
            return block_42(state, tracer, fn_state);
        };
    }
    fn block_42<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_42_0: const #0s : i
        let s_42_0: i128 = 0;
        // D s_42_1: write-var Vt <= s_42_0
        fn_state.Vt = s_42_0;
        // N s_42_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_43<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_43_0: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_44<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_44_0: read-var Vb:i
        let s_44_0: i128 = fn_state.Vb;
        // D s_44_1: write-var Vt <= s_44_0
        fn_state.Vt = s_44_0;
        // N s_44_2: jump b43
        return block_43(state, tracer, fn_state);
    }
    fn block_45<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_45_0: read-var ga#1616:u8
        let s_45_0: u8 = fn_state.ga_1616;
        // D s_45_1: cast zx s_45_0 -> bv
        let s_45_1: Bits = Bits::new(s_45_0 as u128, 2u16);
        // C s_45_2: const #1u : u8
        let s_45_2: u8 = 1;
        // C s_45_3: cast zx s_45_2 -> bv
        let s_45_3: Bits = Bits::new(s_45_2 as u128, 2u16);
        // D s_45_4: cmp-eq s_45_1 s_45_3
        let s_45_4: bool = ((s_45_1) == (s_45_3));
        // D s_45_5: not s_45_4
        let s_45_5: bool = !s_45_4;
        // N s_45_6: branch s_45_5 b47 b46
        if s_45_5 {
            return block_47(state, tracer, fn_state);
        } else {
            return block_46(state, tracer, fn_state);
        };
    }
    fn block_46<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_46_0: read-var T:i64
        let s_46_0: i64 = fn_state.T;
        // D s_46_1: cast zx s_46_0 -> i
        let s_46_1: i128 = (i128::try_from(s_46_0).unwrap());
        // D s_46_2: read-var Vb:i
        let s_46_2: i128 = fn_state.Vb;
        // D s_46_3: cmp-eq s_46_2 s_46_1
        let s_46_3: bool = ((s_46_2) == (s_46_1));
        // D s_46_4: write-var Vc <= s_46_3
        fn_state.Vc = s_46_3;
        // N s_46_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_47<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_47_0: read-var ga#1616:u8
        let s_47_0: u8 = fn_state.ga_1616;
        // D s_47_1: cast zx s_47_0 -> bv
        let s_47_1: Bits = Bits::new(s_47_0 as u128, 2u16);
        // C s_47_2: const #2u : u8
        let s_47_2: u8 = 2;
        // C s_47_3: cast zx s_47_2 -> bv
        let s_47_3: Bits = Bits::new(s_47_2 as u128, 2u16);
        // D s_47_4: cmp-eq s_47_1 s_47_3
        let s_47_4: bool = ((s_47_1) == (s_47_3));
        // D s_47_5: not s_47_4
        let s_47_5: bool = !s_47_4;
        // N s_47_6: branch s_47_5 b49 b48
        if s_47_5 {
            return block_49(state, tracer, fn_state);
        } else {
            return block_48(state, tracer, fn_state);
        };
    }
    fn block_48<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_48_0: read-var T:i64
        let s_48_0: i64 = fn_state.T;
        // D s_48_1: cast zx s_48_0 -> i
        let s_48_1: i128 = (i128::try_from(s_48_0).unwrap());
        // D s_48_2: read-var Vb:i
        let s_48_2: i128 = fn_state.Vb;
        // D s_48_3: cmp-ge s_48_2 s_48_1
        let s_48_3: bool = ((s_48_2) >= (s_48_1));
        // D s_48_4: write-var Vc <= s_48_3
        fn_state.Vc = s_48_3;
        // N s_48_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_49<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_49_0: read-var ga#1616:u8
        let s_49_0: u8 = fn_state.ga_1616;
        // D s_49_1: cast zx s_49_0 -> bv
        let s_49_1: Bits = Bits::new(s_49_0 as u128, 2u16);
        // C s_49_2: const #3u : u8
        let s_49_2: u8 = 3;
        // C s_49_3: cast zx s_49_2 -> bv
        let s_49_3: Bits = Bits::new(s_49_2 as u128, 2u16);
        // D s_49_4: cmp-eq s_49_1 s_49_3
        let s_49_4: bool = ((s_49_1) == (s_49_3));
        // D s_49_5: not s_49_4
        let s_49_5: bool = !s_49_4;
        // N s_49_6: branch s_49_5 b51 b50
        if s_49_5 {
            return block_51(state, tracer, fn_state);
        } else {
            return block_50(state, tracer, fn_state);
        };
    }
    fn block_50<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_50_0: read-var T:i64
        let s_50_0: i64 = fn_state.T;
        // D s_50_1: cast zx s_50_0 -> i
        let s_50_1: i128 = (i128::try_from(s_50_0).unwrap());
        // D s_50_2: read-var Vb:i
        let s_50_2: i128 = fn_state.Vb;
        // D s_50_3: cmp-lt s_50_2 s_50_1
        let s_50_3: bool = ((s_50_2) < (s_50_1));
        // D s_50_4: write-var Vc <= s_50_3
        fn_state.Vc = s_50_3;
        // N s_50_5: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_51<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // N s_51_0: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_52<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // D s_52_0: read-var Vb:i
        let s_52_0: i128 = fn_state.Vb;
        // D s_52_1: write-var return_value <= s_52_0
        fn_state.return_value = s_52_0;
        // N s_52_2: jump b14
        return block_14(state, tracer, fn_state);
    }
    fn block_53<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> i128 {
        // C s_53_0: const #1u : u8
        let s_53_0: bool = true;
        // D s_53_1: write-var gs#2735 <= s_53_0
        fn_state.gs_2735 = s_53_0;
        // N s_53_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
