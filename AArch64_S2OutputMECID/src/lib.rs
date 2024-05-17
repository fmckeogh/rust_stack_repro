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
use u_get_VMECID_A_EL2_Type_MECID::*;
use u_get_VMECID_P_EL2_Type_MECID::*;
use common::*;
pub fn AArch64_S2OutputMECID<T: Tracer>(
    state: &mut State,
    tracer: &T,
    walkparams: ProductTypeb05ce25a107f0c5e,
    paspace: u32,
    descriptor: Bits,
) -> u16 {
    #[derive(Default)]
    struct FunctionState {
        return_value: u16,
        gs_19005: bool,
        descriptor_amec: bool,
        walkparams: ProductTypeb05ce25a107f0c5e,
        paspace: u32,
        descriptor: Bits,
    }
    let fn_state = FunctionState {
        walkparams,
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
        // N s_0_7: branch s_0_6 b13 b1
        if s_0_6 {
            return block_13(state, tracer, fn_state);
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
        // D s_1_7: write-var gs#19005 <= s_1_6
        fn_state.gs_19005 = s_1_6;
        // N s_1_8: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_2_0: read-var gs#19005:u8
        let s_2_0: bool = fn_state.gs_19005;
        // N s_2_1: assert s_2_0
        let s_2_1: () = assert!(s_2_0);
        // D s_2_2: read-var walkparams.5:struct
        let s_2_2: bool = fn_state.walkparams._5;
        // D s_2_3: cast zx s_2_2 -> bv
        let s_2_3: Bits = Bits::new(s_2_2 as u128, 1u16);
        // C s_2_4: const #0u : u8
        let s_2_4: bool = false;
        // C s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // D s_2_6: cmp-eq s_2_3 s_2_5
        let s_2_6: bool = ((s_2_3) == (s_2_5));
        // N s_2_7: branch s_2_6 b12 b3
        if s_2_6 {
            return block_12(state, tracer, fn_state);
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
        // N s_3_3: branch s_3_2 b11 b4
        if s_3_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_4_0: read-var walkparams.2:struct
        let s_4_0: bool = fn_state.walkparams._2;
        // D s_4_1: cast zx s_4_0 -> bv
        let s_4_1: Bits = Bits::new(s_4_0 as u128, 1u16);
        // C s_4_2: const #1u : u8
        let s_4_2: bool = true;
        // C s_4_3: cast zx s_4_2 -> bv
        let s_4_3: Bits = Bits::new(s_4_2 as u128, 1u16);
        // D s_4_4: cmp-eq s_4_1 s_4_3
        let s_4_4: bool = ((s_4_1) == (s_4_3));
        // N s_4_5: branch s_4_4 b10 b5
        if s_4_4 {
            return block_10(state, tracer, fn_state);
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
        // D s_6_0: read-var descriptor_amec:u8
        let s_6_0: bool = fn_state.descriptor_amec;
        // D s_6_1: cast zx s_6_0 -> bv
        let s_6_1: Bits = Bits::new(s_6_0 as u128, 1u16);
        // C s_6_2: const #0u : u8
        let s_6_2: bool = false;
        // C s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // D s_6_4: cmp-eq s_6_1 s_6_3
        let s_6_4: bool = ((s_6_1) == (s_6_3));
        // N s_6_5: branch s_6_4 b9 b7
        if s_6_4 {
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
        // C s_7_0: const #15720u : u32
        let s_7_0: u32 = 15720;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_VMECID_A_EL2_Type_MECID(s_7_1)
        let s_7_2: u16 = u_get_VMECID_A_EL2_Type_MECID(state, tracer, s_7_1);
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
        // C s_9_0: const #20600u : u32
        let s_9_0: u32 = 20600;
        // D s_9_1: read-reg s_9_0:struct
        let s_9_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_9_0 as isize);
            tracer.read_register(s_9_0 as isize, value);
            value
        };
        // D s_9_2: call _get_VMECID_P_EL2_Type_MECID(s_9_1)
        let s_9_2: u16 = u_get_VMECID_P_EL2_Type_MECID(state, tracer, s_9_1);
        // D s_9_3: write-var return_value <= s_9_2
        fn_state.return_value = s_9_2;
        // N s_9_4: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_10_0: read-var descriptor:bv
        let s_10_0: Bits = fn_state.descriptor;
        // D s_10_1: size-of s_10_0
        let s_10_1: u16 = s_10_0.length();
        // D s_10_2: cast zx s_10_1 -> i
        let s_10_2: i128 = (i128::try_from(s_10_1).unwrap());
        // D s_10_3: cast reint s_10_2 -> i64
        let s_10_3: i64 = (s_10_2 as i64);
        // C s_10_4: const #103s : i
        let s_10_4: i128 = 103;
        // D s_10_5: cast zx s_10_3 -> i
        let s_10_5: i128 = (i128::try_from(s_10_3).unwrap());
        // D s_10_6: cmp-lt s_10_4 s_10_5
        let s_10_6: bool = ((s_10_4) < (s_10_5));
        // N s_10_7: assert s_10_6
        let s_10_7: () = assert!(s_10_6);
        // C s_10_8: const #103s : i
        let s_10_8: i128 = 103;
        // D s_10_9: read-var descriptor:bv
        let s_10_9: Bits = fn_state.descriptor;
        // C s_10_10: const #1u : u64
        let s_10_10: u64 = 1;
        // D s_10_11: bit-extract s_10_9 s_10_8 s_10_10
        let s_10_11: Bits = (Bits::new(
            ((s_10_9) >> (s_10_8)).value(),
            u16::try_from(s_10_10).unwrap(),
        ));
        // D s_10_12: cast reint s_10_11 -> u8
        let s_10_12: bool = ((s_10_11.value()) != 0);
        // C s_10_13: const #0s : i
        let s_10_13: i128 = 0;
        // C s_10_14: const #0u : u64
        let s_10_14: u64 = 0;
        // D s_10_15: cast zx s_10_12 -> u64
        let s_10_15: u64 = (s_10_12 as u64);
        // C s_10_16: const #1u : u64
        let s_10_16: u64 = 1;
        // D s_10_17: and s_10_15 s_10_16
        let s_10_17: u64 = ((s_10_15) & (s_10_16));
        // D s_10_18: cmp-eq s_10_17 s_10_16
        let s_10_18: bool = ((s_10_17) == (s_10_16));
        // D s_10_19: lsl s_10_15 s_10_13
        let s_10_19: u64 = s_10_15 << s_10_13;
        // D s_10_20: or s_10_14 s_10_19
        let s_10_20: u64 = ((s_10_14) | (s_10_19));
        // D s_10_21: cmpl s_10_19
        let s_10_21: u64 = !s_10_19;
        // D s_10_22: and s_10_14 s_10_21
        let s_10_22: u64 = ((s_10_14) & (s_10_21));
        // D s_10_23: select s_10_18 s_10_20 s_10_22
        let s_10_23: u64 = if s_10_18 { s_10_20 } else { s_10_22 };
        // D s_10_24: cast trunc s_10_23 -> u8
        let s_10_24: bool = ((s_10_23) != 0);
        // D s_10_25: write-var descriptor_amec <= s_10_24
        fn_state.descriptor_amec = s_10_24;
        // N s_10_26: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_11_0: const #832u : u32
        let s_11_0: u32 = 832;
        // D s_11_1: read-reg s_11_0:u16
        let s_11_1: u16 = {
            let value = state.read_register::<u16>(s_11_0 as isize);
            tracer.read_register(s_11_0 as isize, value);
            value
        };
        // D s_11_2: write-var return_value <= s_11_1
        fn_state.return_value = s_11_1;
        // N s_11_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_12_0: const #832u : u32
        let s_12_0: u32 = 832;
        // D s_12_1: read-reg s_12_0:u16
        let s_12_1: u16 = {
            let value = state.read_register::<u16>(s_12_0 as isize);
            tracer.read_register(s_12_0 as isize, value);
            value
        };
        // D s_12_2: write-var return_value <= s_12_1
        fn_state.return_value = s_12_1;
        // N s_12_3: jump b8
        return block_8(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#19005 <= s_13_0
        fn_state.gs_19005 = s_13_0;
        // N s_13_2: jump b2
        return block_2(state, tracer, fn_state);
    }
}
