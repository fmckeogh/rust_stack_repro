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
use u_get_VMECID_P_EL2_Type_MECID::*;
use common::*;
pub fn AArch64_S1DisabledOutputMECID<T: Tracer>(
    state: &mut State,
    tracer: &T,
    walkparams: ProductTypeef284266e139aee2,
    regime: u32,
    paspace: u32,
) -> u16 {
    #[derive(Default)]
    struct FunctionState {
        gs_17409: bool,
        gs_17410: bool,
        return_value: u16,
        walkparams: ProductTypeef284266e139aee2,
        regime: u32,
        paspace: u32,
    }
    let fn_state = FunctionState {
        walkparams,
        regime,
        paspace,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_0_0: read-var walkparams.10:struct
        let s_0_0: bool = fn_state.walkparams._10;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 1u16);
        // C s_0_2: const #0u : u8
        let s_0_2: bool = false;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b15 b1
        if s_0_4 {
            return block_15(state, tracer, fn_state);
        } else {
            return block_1(state, tracer, fn_state);
        };
    }
    fn block_1<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_1_0: read-var regime:u32
        let s_1_0: u32 = fn_state.regime;
        // C s_1_1: const #2u : u32
        let s_1_1: u32 = 2;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b14 b2
        if s_1_2 {
            return block_14(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_2_0: read-var regime:u32
        let s_2_0: u32 = fn_state.regime;
        // C s_2_1: const #3u : u32
        let s_2_1: u32 = 3;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // N s_2_3: branch s_2_2 b13 b3
        if s_2_2 {
            return block_13(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_3_0: read-var regime:u32
        let s_3_0: u32 = fn_state.regime;
        // C s_3_1: const #4u : u32
        let s_3_1: u32 = 4;
        // D s_3_2: cmp-eq s_3_0 s_3_1
        let s_3_2: bool = ((s_3_0) == (s_3_1));
        // D s_3_3: write-var gs#17409 <= s_3_2
        fn_state.gs_17409 = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_4_0: read-var gs#17409:u8
        let s_4_0: bool = fn_state.gs_17409;
        // D s_4_1: write-var gs#17410 <= s_4_0
        fn_state.gs_17410 = s_4_0;
        // N s_4_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_5_0: read-var gs#17410:u8
        let s_5_0: bool = fn_state.gs_17410;
        // D s_5_1: not s_5_0
        let s_5_1: bool = !s_5_0;
        // N s_5_2: branch s_5_1 b12 b6
        if s_5_1 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_6_0: read-var paspace:u32
        let s_6_0: u32 = fn_state.paspace;
        // C s_6_1: const #3u : u32
        let s_6_1: u32 = 3;
        // D s_6_2: cmp-eq s_6_0 s_6_1
        let s_6_2: bool = ((s_6_0) == (s_6_1));
        // N s_6_3: branch s_6_2 b11 b7
        if s_6_2 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_7_0: read-var regime:u32
        let s_7_0: u32 = fn_state.regime;
        // C s_7_1: const #4u : u32
        let s_7_1: u32 = 4;
        // D s_7_2: cmp-eq s_7_0 s_7_1
        let s_7_2: bool = ((s_7_0) == (s_7_1));
        // N s_7_3: branch s_7_2 b10 b8
        if s_7_2 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_8(state, tracer, fn_state);
        };
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_8_0: const #22904u : u32
        let s_8_0: u32 = 22904;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_MECID_P0_EL2_Type_MECID(s_8_1)
        let s_8_2: u16 = u_get_MECID_P0_EL2_Type_MECID(state, tracer, s_8_1);
        // D s_8_3: write-var return_value <= s_8_2
        fn_state.return_value = s_8_2;
        // N s_8_4: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_9_0: read-var return_value:u16
        let s_9_0: u16 = fn_state.return_value;
        // N s_9_1: return s_9_0
        return s_9_0;
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_10_0: const #20600u : u32
        let s_10_0: u32 = 20600;
        // D s_10_1: read-reg s_10_0:struct
        let s_10_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_10_0 as isize);
            tracer.read_register(s_10_0 as isize, value);
            value
        };
        // D s_10_2: call _get_VMECID_P_EL2_Type_MECID(s_10_1)
        let s_10_2: u16 = u_get_VMECID_P_EL2_Type_MECID(state, tracer, s_10_1);
        // D s_10_3: write-var return_value <= s_10_2
        fn_state.return_value = s_10_2;
        // N s_10_4: jump b9
        return block_9(state, tracer, fn_state);
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
        // N s_11_3: jump b9
        return block_9(state, tracer, fn_state);
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
        // N s_12_3: jump b9
        return block_9(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_13_0: const #1u : u8
        let s_13_0: bool = true;
        // D s_13_1: write-var gs#17409 <= s_13_0
        fn_state.gs_17409 = s_13_0;
        // N s_13_2: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_14<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_14_0: const #1u : u8
        let s_14_0: bool = true;
        // D s_14_1: write-var gs#17410 <= s_14_0
        fn_state.gs_17410 = s_14_0;
        // N s_14_2: jump b5
        return block_5(state, tracer, fn_state);
    }
    fn block_15<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_15_0: const #832u : u32
        let s_15_0: u32 = 832;
        // D s_15_1: read-reg s_15_0:u16
        let s_15_1: u16 = {
            let value = state.read_register::<u16>(s_15_0 as isize);
            tracer.read_register(s_15_0 as isize, value);
            value
        };
        // D s_15_2: write-var return_value <= s_15_1
        fn_state.return_value = s_15_1;
        // N s_15_3: jump b9
        return block_9(state, tracer, fn_state);
    }
}
