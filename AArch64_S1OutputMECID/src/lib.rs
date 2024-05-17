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
use u_get_MECID_P0_EL2_Type_MECID::*;
use u_get_MECID_A0_EL2_Type_MECID::*;
use u_get_MECID_RL_A_EL3_Type_MECID::*;
use u_get_MECID_A1_EL2_Type_MECID::*;
use u_get_VMECID_P_EL2_Type_MECID::*;
use u_get_MECID_P1_EL2_Type_MECID::*;
use common::*;
pub fn AArch64_S1OutputMECID<T: Tracer>(
    state: &mut State,
    tracer: &T,
    walkparams: ProductTypeef284266e139aee2,
    regime: u32,
    varange: u32,
    paspace: u32,
    descriptor: Bits,
) -> u16 {
    #[derive(Default)]
    struct FunctionState {
        gs_17503: bool,
        ga_13057: u16,
        return_value: u16,
        descriptor_amec: bool,
        walkparams: ProductTypeef284266e139aee2,
        regime: u32,
        varange: u32,
        paspace: u32,
        descriptor: Bits,
    }
    let fn_state = FunctionState {
        walkparams,
        regime,
        varange,
        paspace,
        descriptor,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_0_0: read-var descriptor:bv
        let s_0_0: Bits = fn_state.descriptor;
        // D s_0_1: size-of s_0_0
        let s_0_1: u16 = s_0_0.length();
        // D s_0_2: cast zx s_0_1 -> i
        let s_0_2: i128 = (i128::try_from(s_0_1).unwrap());
        // D s_0_3: cast reint s_0_2 -> i64
        let s_0_3: i64 = (s_0_2 as i64);
        // C s_0_4: const #64s : i
        let s_0_4: i128 = 64;
        // D s_0_5: cast zx s_0_3 -> i
        let s_0_5: i128 = (i128::try_from(s_0_3).unwrap());
        // D s_0_6: cmp-eq s_0_5 s_0_4
        let s_0_6: bool = ((s_0_5) == (s_0_4));
        // N s_0_7: branch s_0_6 b27 b1
        if s_0_6 {
            return block_27(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_1_0: read-var descriptor:bv
        let s_1_0: Bits = fn_state.descriptor;
        // D s_1_1: size-of s_1_0
        let s_1_1: u16 = s_1_0.length();
        // D s_1_2: cast zx s_1_1 -> i
        let s_1_2: i128 = (i128::try_from(s_1_1).unwrap());
        // D s_1_3: cast reint s_1_2 -> i64
        let s_1_3: i64 = (s_1_2 as i64);
        // C s_1_4: const #128s : i
        let s_1_4: i128 = 128;
        // D s_1_5: cast zx s_1_3 -> i
        let s_1_5: i128 = (i128::try_from(s_1_3).unwrap());
        // D s_1_6: cmp-eq s_1_5 s_1_4
        let s_1_6: bool = ((s_1_5) == (s_1_4));
        // D s_1_7: write-var gs#17503 <= s_1_6
        fn_state.gs_17503 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_2_0: read-var gs#17503:u8
        let s_2_0: bool = fn_state.gs_17503;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var walkparams.10:struct
        let s_2_2: bool = fn_state.walkparams._10;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #0u : u8
        let s_2_4: bool = false;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b26 b3
        if s_2_6 {
            return block_26(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_3_0: read-var paspace:u32
        let s_3_0: u32 = fn_state.paspace;
        // C s_3_1: const #3u : u32
        let s_3_1: u32 = 3;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // N s_3_3: branch s_3_2 b25 b4
        if s_3_2 {
            return block_25(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_4_0: read-var walkparams.3:struct
        let s_4_0: bool = fn_state.walkparams._3;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b24 b5
        if s_4_4 {
            return block_24(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_5_0: const #63s : i
        let s_5_0: i128 = 63;
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
        // D s_5_17: write-var descriptor_amec <= s_5_16
        fn_state.descriptor_amec = s_5_16;
        // N s_5_18: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_6_0: const #0u : u32
        let s_6_0: u32 = 0;
        // D s_6_1: read-var regime:u32
        let s_6_1: u32 = fn_state.regime;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // D s_6_3: not s_6_2
        let s_6_3: bool = !s_6_2;
        // N s_6_4: branch s_6_3 b9 b7
        if s_6_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_7_0: const #102568u : u32
        let s_7_0: u32 = 102568;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_MECID_RL_A_EL3_Type_MECID(s_7_1)
        let s_7_2: u16 = u_get_MECID_RL_A_EL3_Type_MECID(state, tracer, s_7_1);
        // D s_7_3: write-var return_value <= s_7_2
        fn_state.return_value = s_7_2;
        // N s_7_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_8_0: read-var return_value:u16
        let s_8_0: u16 = fn_state.return_value;
        // N s_8_1: return s_8_0
        return s_8_0;
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_9_0: const #2u : u32
        let s_9_0: u32 = 2;
        // D s_9_1: read-var regime:u32
        let s_9_1: u32 = fn_state.regime;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: not s_9_2
        let s_9_3: bool = !s_9_2;
        // N s_9_4: branch s_9_3 b13 b10
        if s_9_3 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_10_0: read-var descriptor_amec:u8
        let s_10_0: bool = fn_state.descriptor_amec;
        // D s_10_1: cast zx s_10_0 -> bv
        let s_10_1: Bits = Bits::new(s_10_0 as u128, 1u16);
        // C s_10_2: const #0u : u8
        let s_10_2: bool = false;
        // C s_10_3: cast zx s_10_2 -> bv
        let s_10_3: Bits = Bits::new(s_10_2 as u128, 1u16);
        // D s_10_4: cmp-eq s_10_1 s_10_3
        let s_10_4: bool = ((s_10_1) == (s_10_3));
        // N s_10_5: branch s_10_4 b12 b11
        if s_10_4 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_11(state, tracer, fn_state);
        };
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_11_0: const #1592u : u32
        let s_11_0: u32 = 1592;
        // D s_11_1: read-reg s_11_0:struct
        let s_11_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: call _get_MECID_A0_EL2_Type_MECID(s_11_1)
        let s_11_2: u16 = u_get_MECID_A0_EL2_Type_MECID(state, tracer, s_11_1);
        // D s_11_3: write-var return_value <= s_11_2
        fn_state.return_value = s_11_2;
        // N s_11_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_12_0: const #22904u : u32
        let s_12_0: u32 = 22904;
        // D s_12_1: read-reg s_12_0:struct
        let s_12_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: call _get_MECID_P0_EL2_Type_MECID(s_12_1)
        let s_12_2: u16 = u_get_MECID_P0_EL2_Type_MECID(state, tracer, s_12_1);
        // D s_12_3: write-var return_value <= s_12_2
        fn_state.return_value = s_12_2;
        // N s_12_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_13_0: const #3u : u32
        let s_13_0: u32 = 3;
        // D s_13_1: read-var regime:u32
        let s_13_1: u32 = fn_state.regime;
        // D s_13_2: cmp-eq s_13_0 s_13_1
        let s_13_2: bool = ((s_13_0) == (s_13_1));
        // D s_13_3: not s_13_2
        let s_13_3: bool = !s_13_2;
        // N s_13_4: branch s_13_3 b21 b14
        if s_13_3 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_14_0: read-var varange:u32
        let s_14_0: u32 = fn_state.varange;
        // C s_14_1: const #0u : u32
        let s_14_1: u32 = 0;
        // D s_14_2: cmp-eq s_14_0 s_14_1
        let s_14_2: bool = ((s_14_0) == (s_14_1));
        // N s_14_3: branch s_14_2 b18 b15
        if s_14_2 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_15(state, tracer, fn_state);
        };
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_15_0: read-var descriptor_amec:u8
        let s_15_0: bool = fn_state.descriptor_amec;
        // D s_15_1: cast zx s_15_0 -> bv
        let s_15_1: Bits = Bits::new(s_15_0 as u128, 1u16);
        // C s_15_2: const #0u : u8
        let s_15_2: bool = false;
        // C s_15_3: cast zx s_15_2 -> bv
        let s_15_3: Bits = Bits::new(s_15_2 as u128, 1u16);
        // D s_15_4: cmp-eq s_15_1 s_15_3
        let s_15_4: bool = ((s_15_1) == (s_15_3));
        // N s_15_5: branch s_15_4 b17 b16
        if s_15_4 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_16(state, tracer, fn_state);
        };
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_16_0: const #23112u : u32
        let s_16_0: u32 = 23112;
        // D s_16_1: read-reg s_16_0:struct
        let s_16_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_16_0 as isize);
            tracer.read_register(s_16_0 as isize, value);
            value
        };
        // D s_16_2: call _get_MECID_A1_EL2_Type_MECID(s_16_1)
        let s_16_2: u16 = u_get_MECID_A1_EL2_Type_MECID(state, tracer, s_16_1);
        // D s_16_3: write-var return_value <= s_16_2
        fn_state.return_value = s_16_2;
        // N s_16_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_17_0: const #90168u : u32
        let s_17_0: u32 = 90168;
        // D s_17_1: read-reg s_17_0:struct
        let s_17_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_17_0 as isize);
            tracer.read_register(s_17_0 as isize, value);
            value
        };
        // D s_17_2: call _get_MECID_P1_EL2_Type_MECID(s_17_1)
        let s_17_2: u16 = u_get_MECID_P1_EL2_Type_MECID(state, tracer, s_17_1);
        // D s_17_3: write-var return_value <= s_17_2
        fn_state.return_value = s_17_2;
        // N s_17_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_18_0: read-var descriptor_amec:u8
        let s_18_0: bool = fn_state.descriptor_amec;
        // D s_18_1: cast zx s_18_0 -> bv
        let s_18_1: Bits = Bits::new(s_18_0 as u128, 1u16);
        // C s_18_2: const #0u : u8
        let s_18_2: bool = false;
        // C s_18_3: cast zx s_18_2 -> bv
        let s_18_3: Bits = Bits::new(s_18_2 as u128, 1u16);
        // D s_18_4: cmp-eq s_18_1 s_18_3
        let s_18_4: bool = ((s_18_1) == (s_18_3));
        // N s_18_5: branch s_18_4 b20 b19
        if s_18_4 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_19(state, tracer, fn_state);
        };
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_19_0: const #1592u : u32
        let s_19_0: u32 = 1592;
        // D s_19_1: read-reg s_19_0:struct
        let s_19_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_19_0 as isize);
            tracer.read_register(s_19_0 as isize, value);
            value
        };
        // D s_19_2: call _get_MECID_A0_EL2_Type_MECID(s_19_1)
        let s_19_2: u16 = u_get_MECID_A0_EL2_Type_MECID(state, tracer, s_19_1);
        // D s_19_3: write-var return_value <= s_19_2
        fn_state.return_value = s_19_2;
        // N s_19_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_20_0: const #22904u : u32
        let s_20_0: u32 = 22904;
        // D s_20_1: read-reg s_20_0:struct
        let s_20_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_20_0 as isize);
            tracer.read_register(s_20_0 as isize, value);
            value
        };
        // D s_20_2: call _get_MECID_P0_EL2_Type_MECID(s_20_1)
        let s_20_2: u16 = u_get_MECID_P0_EL2_Type_MECID(state, tracer, s_20_1);
        // D s_20_3: write-var return_value <= s_20_2
        fn_state.return_value = s_20_2;
        // N s_20_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_21_0: const #4u : u32
        let s_21_0: u32 = 4;
        // D s_21_1: read-var regime:u32
        let s_21_1: u32 = fn_state.regime;
        // D s_21_2: cmp-eq s_21_0 s_21_1
        let s_21_2: bool = ((s_21_0) == (s_21_1));
        // D s_21_3: not s_21_2
        let s_21_3: bool = !s_21_2;
        // N s_21_4: branch s_21_3 b23 b22
        if s_21_3 {
            return block_23(state, tracer, fn_state);
        } else {
            return block_22(state, tracer, fn_state);
        };
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_22_0: const #20600u : u32
        let s_22_0: u32 = 20600;
        // D s_22_1: read-reg s_22_0:struct
        let s_22_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call _get_VMECID_P_EL2_Type_MECID(s_22_1)
        let s_22_2: u16 = u_get_VMECID_P_EL2_Type_MECID(state, tracer, s_22_1);
        // D s_22_3: write-var return_value <= s_22_2
        fn_state.return_value = s_22_2;
        // N s_22_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_23<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_23_0: read-var ga#13057:u16
        let s_23_0: u16 = fn_state.ga_13057;
        // D s_23_1: write-var return_value <= s_23_0
        fn_state.return_value = s_23_0;
        // N s_23_2: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_24<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_24_0: read-var descriptor:bv
        let s_24_0: Bits = fn_state.descriptor;
        // D s_24_1: size-of s_24_0
        let s_24_1: u16 = s_24_0.length();
        // D s_24_2: cast zx s_24_1 -> i
        let s_24_2: i128 = (i128::try_from(s_24_1).unwrap());
        // D s_24_3: cast reint s_24_2 -> i64
        let s_24_3: i64 = (s_24_2 as i64);
        // C s_24_4: const #103s : i
        let s_24_4: i128 = 103;
        // D s_24_5: cast zx s_24_3 -> i
        let s_24_5: i128 = (i128::try_from(s_24_3).unwrap());
        // D s_24_6: cmp-lt s_24_4 s_24_5
        let s_24_6: bool = ((s_24_4) < (s_24_5));
        // N s_24_7: assert s_24_6
        let s_24_7: () = assert!(s_24_6);
        // C s_24_8: const #103s : i
        let s_24_8: i128 = 103;
        // D s_24_9: read-var descriptor:bv
        let s_24_9: Bits = fn_state.descriptor;
        // C s_24_10: const #1u : u64
        let s_24_10: u64 = 1;
        // D s_24_11: bit-extract s_24_9 s_24_8 s_24_10
        let s_24_11: Bits = (Bits::new(
            ((s_24_9) >> (s_24_8)).value(),
            u16::try_from(s_24_10).unwrap(),
        ));
        // D s_24_12: cast reint s_24_11 -> u8
        let s_24_12: bool = ((s_24_11.value()) != 0);
        // C s_24_13: const #0s : i
        let s_24_13: i128 = 0;
        // C s_24_14: const #0u : u64
        let s_24_14: u64 = 0;
        // D s_24_15: cast zx s_24_12 -> u64
        let s_24_15: u64 = (s_24_12 as u64);
        // C s_24_16: const #1u : u64
        let s_24_16: u64 = 1;
        // D s_24_17: and s_24_15 s_24_16
        let s_24_17: u64 = ((s_24_15) & (s_24_16));
        // D s_24_18: cmp-eq s_24_17 s_24_16
        let s_24_18: bool = ((s_24_17) == (s_24_16));
        // D s_24_19: lsl s_24_15 s_24_13
        let s_24_19: u64 = s_24_15 << s_24_13;
        // D s_24_20: or s_24_14 s_24_19
        let s_24_20: u64 = ((s_24_14) | (s_24_19));
        // D s_24_21: cmpl s_24_19
        let s_24_21: u64 = !s_24_19;
        // D s_24_22: and s_24_14 s_24_21
        let s_24_22: u64 = ((s_24_14) & (s_24_21));
        // D s_24_23: select s_24_18 s_24_20 s_24_22
        let s_24_23: u64 = if s_24_18 { s_24_20 } else { s_24_22 };
        // D s_24_24: cast trunc s_24_23 -> u8
        let s_24_24: bool = ((s_24_23) != 0);
        // D s_24_25: write-var descriptor_amec <= s_24_24
        fn_state.descriptor_amec = s_24_24;
        // N s_24_26: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_25<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_25_0: const #832u : u32
        let s_25_0: u32 = 832;
        // D s_25_1: read-reg s_25_0:u16
        let s_25_1: u16 = {
            let value = state.read_register::<u16>(s_25_0 as isize);
            tracer.read_register(s_25_0 as isize, value);
            value
        };
        // D s_25_2: write-var return_value <= s_25_1
        fn_state.return_value = s_25_1;
        // N s_25_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_26<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_26_0: const #832u : u32
        let s_26_0: u32 = 832;
        // D s_26_1: read-reg s_26_0:u16
        let s_26_1: u16 = {
            let value = state.read_register::<u16>(s_26_0 as isize);
            tracer.read_register(s_26_0 as isize, value);
            value
        };
        // D s_26_2: write-var return_value <= s_26_1
        fn_state.return_value = s_26_1;
        // N s_26_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_27<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_27_0: const #1u : u8
        let s_27_0: bool = true;
        // D s_27_1: write-var gs#17503 <= s_27_0
        fn_state.gs_17503 = s_27_0;
        // N s_27_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
