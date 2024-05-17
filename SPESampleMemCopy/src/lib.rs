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
use u_get_PMSFCR_EL1_Type_ST::*;
use u_get_PMSFCR_EL1_Type_LD::*;
use u_get_PMSFCR_EL1_Type_FT::*;
use SPEGetRandomBoolean::*;
use common::*;
pub fn SPESampleMemCopy<T: Tracer>(state: &mut State, tracer: &T, gs_26281: ()) -> () {
    #[derive(Default)]
    struct FunctionState {
        stores_pass_filter: bool,
        loads_pass_filter: bool,
        gs_26282: bool,
        record_load: bool,
        gs_26283: bool,
        gs_26287: bool,
        gs_26284: bool,
        ldst: bool,
        gs_26281: (),
    }
    let fn_state = FunctionState {
        gs_26281,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_0_0: const #101208u : u32
        let s_0_0: u32 = 101208;
        // D s_0_1: read-reg s_0_0:struct
        let s_0_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_0_0 as isize);
            tracer.read_register(s_0_0 as isize, value);
            value
        };
        // D s_0_2: call _get_PMSFCR_EL1_Type_FT(s_0_1)
        let s_0_2: bool = u_get_PMSFCR_EL1_Type_FT(state, tracer, s_0_1);
        // D s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // C s_0_4: const #1u : u8
        let s_0_4: bool = true;
        // C s_0_5: cast zx s_0_4 -> bv
        let s_0_5: Bits = Bits::new(s_0_4 as u128, 1u16);
        // D s_0_6: cmp-eq s_0_3 s_0_5
        let s_0_6: bool = ((s_0_3) == (s_0_5));
        // N s_0_7: branch s_0_6 b22 b1
        if s_0_6 {
            return block_22(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#26282 <= s_1_0
        fn_state.gs_26282 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_2_0: read-var gs#26282:u8
        let s_2_0: bool = fn_state.gs_26282;
        // D s_2_1: write-var loads_pass_filter <= s_2_0
        fn_state.loads_pass_filter = s_2_0;
        // C s_2_2: const #101208u : u32
        let s_2_2: u32 = 101208;
        // D s_2_3: read-reg s_2_2:struct
        let s_2_3: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_2_2 as isize);
            tracer.read_register(s_2_2 as isize, value);
            value
        };
        // D s_2_4: call _get_PMSFCR_EL1_Type_FT(s_2_3)
        let s_2_4: bool = u_get_PMSFCR_EL1_Type_FT(state, tracer, s_2_3);
        // D s_2_5: cast zx s_2_4 -> bv
        let s_2_5: Bits = Bits::new(s_2_4 as u128, 1u16);
        // C s_2_6: const #1u : u8
        let s_2_6: bool = true;
        // C s_2_7: cast zx s_2_6 -> bv
        let s_2_7: Bits = Bits::new(s_2_6 as u128, 1u16);
        // D s_2_8: cmp-eq s_2_5 s_2_7
        let s_2_8: bool = ((s_2_5) == (s_2_7));
        // N s_2_9: branch s_2_8 b21 b3
        if s_2_8 {
            return block_21(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_3_0: const #0u : u8
        let s_3_0: bool = false;
        // D s_3_1: write-var gs#26283 <= s_3_0
        fn_state.gs_26283 = s_3_0;
        // N s_3_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_4_0: read-var gs#26283:u8
        let s_4_0: bool = fn_state.gs_26283;
        // D s_4_1: write-var stores_pass_filter <= s_4_0
        fn_state.stores_pass_filter = s_4_0;
        // D s_4_2: read-var loads_pass_filter:u8
        let s_4_2: bool = fn_state.loads_pass_filter;
        // N s_4_3: branch s_4_2 b20 b5
        if s_4_2 {
            return block_20(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_5_0: const #0u : u8
        let s_5_0: bool = false;
        // D s_5_1: write-var gs#26284 <= s_5_0
        fn_state.gs_26284 = s_5_0;
        // N s_5_2: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_6_0: read-var gs#26284:u8
        let s_6_0: bool = fn_state.gs_26284;
        // N s_6_1: branch s_6_0 b19 b7
        if s_6_0 {
            return block_19(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_7_0: read-var loads_pass_filter:u8
        let s_7_0: bool = fn_state.loads_pass_filter;
        // D s_7_1: not s_7_0
        let s_7_1: bool = !s_7_0;
        // N s_7_2: branch s_7_1 b18 b8
        if s_7_1 {
            return block_18(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_8_0: const #0u : u8
        let s_8_0: bool = false;
        // D s_8_1: write-var gs#26287 <= s_8_0
        fn_state.gs_26287 = s_8_0;
        // N s_8_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_9_0: read-var gs#26287:u8
        let s_9_0: bool = fn_state.gs_26287;
        // N s_9_1: branch s_9_0 b17 b10
        if s_9_0 {
            return block_17(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_10_0: const #() : ()
        let s_10_0: () = ();
        // S s_10_1: call SPEGetRandomBoolean(s_10_0)
        let s_10_1: bool = SPEGetRandomBoolean(state, tracer, s_10_0);
        // D s_10_2: write-var record_load <= s_10_1
        fn_state.record_load = s_10_1;
        // N s_10_3: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_11_0: const #1u : u8
        let s_11_0: u8 = 1;
        // C s_11_1: const #17136u : u32
        let s_11_1: u32 = 17136;
        // N s_11_2: write-reg s_11_1 <= s_11_0
        let s_11_2: () = {
            state.write_register::<u8>(s_11_1 as isize, s_11_0);
            tracer.write_register(s_11_1 as isize, s_11_0);
        };
        // D s_11_3: read-var record_load:u8
        let s_11_3: bool = fn_state.record_load;
        // N s_11_4: branch s_11_3 b16 b12
        if s_11_3 {
            return block_16(state, tracer, fn_state);
        } else {
            return block_12(state, tracer, fn_state);
        };
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_12_0: const #1u : u8
        let s_12_0: bool = true;
        // D s_12_1: write-var ldst <= s_12_0
        fn_state.ldst = s_12_0;
        // N s_12_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_13_0: const #16u : u8
        let s_13_0: u8 = 16;
        // C s_13_1: cast zx s_13_0 -> bv
        let s_13_1: Bits = Bits::new(s_13_0 as u128, 7u16);
        // D s_13_2: read-var ldst:u8
        let s_13_2: bool = fn_state.ldst;
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 1u16);
        // C s_13_4: cast reint s_13_1 -> u128
        let s_13_4: u128 = (s_13_1.value() as u128);
        // D s_13_5: size-of s_13_1
        let s_13_5: u16 = s_13_1.length();
        // D s_13_6: cast reint s_13_3 -> u128
        let s_13_6: u128 = (s_13_3.value() as u128);
        // D s_13_7: size-of s_13_3
        let s_13_7: u16 = s_13_3.length();
        // D s_13_8: lsl s_13_4 s_13_7
        let s_13_8: u128 = s_13_4 << s_13_7;
        // D s_13_9: or s_13_8 s_13_6
        let s_13_9: u128 = ((s_13_8) | (s_13_6));
        // D s_13_10: add s_13_5 s_13_7
        let s_13_10: u16 = (s_13_5 + s_13_7);
        // D s_13_11: create-bits s_13_9 s_13_10
        let s_13_11: Bits = Bits::new(s_13_9, s_13_10);
        // D s_13_12: cast reint s_13_11 -> u8
        let s_13_12: u8 = (s_13_11.value() as u8);
        // C s_13_13: const #0s : i
        let s_13_13: i128 = 0;
        // C s_13_14: const #13528u : u32
        let s_13_14: u32 = 13528;
        // D s_13_15: read-reg s_13_14:u8
        let s_13_15: u8 = {
            let value = state.read_register::<u8>(s_13_14 as isize);
            tracer.read_register(s_13_14 as isize, value);
            value
        };
        // D s_13_16: cast zx s_13_15 -> bv
        let s_13_16: Bits = Bits::new(s_13_15 as u128, 8u16);
        // D s_13_17: cast zx s_13_12 -> bv
        let s_13_17: Bits = Bits::new(s_13_12 as u128, 8u16);
        // C s_13_18: const #7s : i
        let s_13_18: i128 = 7;
        // C s_13_19: const #1u : u64
        let s_13_19: u64 = 1;
        // C s_13_20: cast zx s_13_19 -> bv
        let s_13_20: Bits = Bits::new(s_13_19 as u128, 64u16);
        // C s_13_21: lsl s_13_20 s_13_18
        let s_13_21: Bits = s_13_20 << s_13_18;
        // C s_13_22: sub s_13_21 s_13_20
        let s_13_22: Bits = ((s_13_21) - (s_13_20));
        // D s_13_23: and s_13_17 s_13_22
        let s_13_23: Bits = ((s_13_17) & (s_13_22));
        // D s_13_24: lsl s_13_23 s_13_13
        let s_13_24: Bits = s_13_23 << s_13_13;
        // C s_13_25: lsl s_13_22 s_13_13
        let s_13_25: Bits = s_13_22 << s_13_13;
        // C s_13_26: cmpl s_13_25
        let s_13_26: Bits = !s_13_25;
        // D s_13_27: and s_13_16 s_13_26
        let s_13_27: Bits = ((s_13_16) & (s_13_26));
        // D s_13_28: or s_13_27 s_13_24
        let s_13_28: Bits = ((s_13_27) | (s_13_24));
        // D s_13_29: cast reint s_13_28 -> u8
        let s_13_29: u8 = (s_13_28.value() as u8);
        // C s_13_30: const #13528u : u32
        let s_13_30: u32 = 13528;
        // N s_13_31: write-reg s_13_30 <= s_13_29
        let s_13_31: () = {
            state.write_register::<u8>(s_13_30 as isize, s_13_29);
            tracer.write_register(s_13_30 as isize, s_13_29);
        };
        // C s_13_32: const #1u : u8
        let s_13_32: bool = true;
        // C s_13_33: const #11528u : u32
        let s_13_33: u32 = 11528;
        // N s_13_34: write-reg s_13_33 <= s_13_32
        let s_13_34: () = {
            state.write_register::<bool>(s_13_33 as isize, s_13_32);
            tracer.write_register(s_13_33 as isize, s_13_32);
        };
        // D s_13_35: read-var record_load:u8
        let s_13_35: bool = fn_state.record_load;
        // N s_13_36: branch s_13_35 b15 b14
        if s_13_35 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_14(state, tracer, fn_state);
        };
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_14_0: const #1u : u32
        let s_14_0: u32 = 1;
        // C s_14_1: const #19040u : u32
        let s_14_1: u32 = 19040;
        // N s_14_2: write-reg s_14_1 <= s_14_0
        let s_14_2: () = {
            state.write_register::<u32>(s_14_1 as isize, s_14_0);
            tracer.write_register(s_14_1 as isize, s_14_0);
        };
        // N s_14_3: return
        return;
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_15_0: const #0u : u32
        let s_15_0: u32 = 0;
        // C s_15_1: const #19040u : u32
        let s_15_1: u32 = 19040;
        // N s_15_2: write-reg s_15_1 <= s_15_0
        let s_15_2: () = {
            state.write_register::<u32>(s_15_1 as isize, s_15_0);
            tracer.write_register(s_15_1 as isize, s_15_0);
        };
        // N s_15_3: return
        return;
    }
    fn block_16<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_16_0: const #0u : u8
        let s_16_0: bool = false;
        // D s_16_1: write-var ldst <= s_16_0
        fn_state.ldst = s_16_0;
        // N s_16_2: jump b13
        return block_13(state, tracer, fn_state);
    }
    fn block_17<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_17_0: const #0u : u8
        let s_17_0: bool = false;
        // D s_17_1: write-var record_load <= s_17_0
        fn_state.record_load = s_17_0;
        // N s_17_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_18<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_18_0: read-var stores_pass_filter:u8
        let s_18_0: bool = fn_state.stores_pass_filter;
        // D s_18_1: write-var gs#26287 <= s_18_0
        fn_state.gs_26287 = s_18_0;
        // N s_18_2: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_19<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_19_0: const #1u : u8
        let s_19_0: bool = true;
        // D s_19_1: write-var record_load <= s_19_0
        fn_state.record_load = s_19_0;
        // N s_19_2: jump b11
        return block_11(state, tracer, fn_state);
    }
    fn block_20<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // D s_20_0: read-var stores_pass_filter:u8
        let s_20_0: bool = fn_state.stores_pass_filter;
        // D s_20_1: not s_20_0
        let s_20_1: bool = !s_20_0;
        // D s_20_2: write-var gs#26284 <= s_20_1
        fn_state.gs_26284 = s_20_1;
        // N s_20_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_21<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_21_0: const #101208u : u32
        let s_21_0: u32 = 101208;
        // D s_21_1: read-reg s_21_0:struct
        let s_21_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_21_0 as isize);
            tracer.read_register(s_21_0 as isize, value);
            value
        };
        // D s_21_2: call _get_PMSFCR_EL1_Type_ST(s_21_1)
        let s_21_2: bool = u_get_PMSFCR_EL1_Type_ST(state, tracer, s_21_1);
        // D s_21_3: cast zx s_21_2 -> bv
        let s_21_3: Bits = Bits::new(s_21_2 as u128, 1u16);
        // C s_21_4: const #1u : u8
        let s_21_4: bool = true;
        // C s_21_5: cast zx s_21_4 -> bv
        let s_21_5: Bits = Bits::new(s_21_4 as u128, 1u16);
        // D s_21_6: cmp-eq s_21_3 s_21_5
        let s_21_6: bool = ((s_21_3) == (s_21_5));
        // D s_21_7: write-var gs#26283 <= s_21_6
        fn_state.gs_26283 = s_21_6;
        // N s_21_8: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_22<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> () {
        // C s_22_0: const #101208u : u32
        let s_22_0: u32 = 101208;
        // D s_22_1: read-reg s_22_0:struct
        let s_22_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_22_0 as isize);
            tracer.read_register(s_22_0 as isize, value);
            value
        };
        // D s_22_2: call _get_PMSFCR_EL1_Type_LD(s_22_1)
        let s_22_2: bool = u_get_PMSFCR_EL1_Type_LD(state, tracer, s_22_1);
        // D s_22_3: cast zx s_22_2 -> bv
        let s_22_3: Bits = Bits::new(s_22_2 as u128, 1u16);
        // C s_22_4: const #1u : u8
        let s_22_4: bool = true;
        // C s_22_5: cast zx s_22_4 -> bv
        let s_22_5: Bits = Bits::new(s_22_4 as u128, 1u16);
        // D s_22_6: cmp-eq s_22_3 s_22_5
        let s_22_6: bool = ((s_22_3) == (s_22_5));
        // D s_22_7: write-var gs#26282 <= s_22_6
        fn_state.gs_26282 = s_22_6;
        // N s_22_8: jump b2
        return block_2(state, tracer, fn_state);
    }
}
