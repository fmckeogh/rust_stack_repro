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
use u_get_TCR_EL2_Type_A1::*;
use u_get_VMECID_P_EL2_Type_MECID::*;
use u_get_MECID_P1_EL2_Type_MECID::*;
use Unreachable::*;
use common::*;
pub fn AArch64_TTWalkMECID<T: Tracer>(
    state: &mut State,
    tracer: &T,
    emec: bool,
    regime: u32,
    ss: u32,
) -> u16 {
    #[derive(Default)]
    struct FunctionState {
        ga_14531: u16,
        return_value: u16,
        emec: bool,
        regime: u32,
        ss: u32,
    }
    let fn_state = FunctionState {
        emec,
        regime,
        ss,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_0_0: read-var emec:u8
        let s_0_0: bool = fn_state.emec;
        // D s_0_1: cast zx s_0_0 -> bv
        let s_0_1: Bits = Bits::new(s_0_0 as u128, 1u16);
        // C s_0_2: const #0u : u8
        let s_0_2: bool = false;
        // C s_0_3: cast zx s_0_2 -> bv
        let s_0_3: Bits = Bits::new(s_0_2 as u128, 1u16);
        // D s_0_4: cmp-eq s_0_1 s_0_3
        let s_0_4: bool = ((s_0_1) == (s_0_3));
        // N s_0_5: branch s_0_4 b13 b1
        if s_0_4 {
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
        // D s_1_0: read-var ss:u32
        let s_1_0: u32 = fn_state.ss;
        // C s_1_1: const #2u : u32
        let s_1_1: u32 = 2;
        // D s_1_2: cmp-eq s_1_0 s_1_1
        let s_1_2: bool = ((s_1_0) == (s_1_1));
        // N s_1_3: branch s_1_2 b12 b2
        if s_1_2 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_2(state, tracer, fn_state);
        };
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_2_0: const #2u : u32
        let s_2_0: u32 = 2;
        // D s_2_1: read-var regime:u32
        let s_2_1: u32 = fn_state.regime;
        // D s_2_2: cmp-eq s_2_0 s_2_1
        let s_2_2: bool = ((s_2_0) == (s_2_1));
        // D s_2_3: not s_2_2
        let s_2_3: bool = !s_2_2;
        // N s_2_4: branch s_2_3 b5 b3
        if s_2_3 {
            return block_5(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_3_0: const #22904u : u32
        let s_3_0: u32 = 22904;
        // D s_3_1: read-reg s_3_0:struct
        let s_3_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_3_0 as isize);
            tracer.read_register(s_3_0 as isize, value);
            value
        };
        // D s_3_2: call _get_MECID_P0_EL2_Type_MECID(s_3_1)
        let s_3_2: u16 = u_get_MECID_P0_EL2_Type_MECID(state, tracer, s_3_1);
        // D s_3_3: write-var return_value <= s_3_2
        fn_state.return_value = s_3_2;
        // N s_3_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // D s_4_0: read-var return_value:u16
        let s_4_0: u16 = fn_state.return_value;
        // N s_4_1: return s_4_0
        return s_4_0;
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_5_0: const #3u : u32
        let s_5_0: u32 = 3;
        // D s_5_1: read-var regime:u32
        let s_5_1: u32 = fn_state.regime;
        // D s_5_2: cmp-eq s_5_0 s_5_1
        let s_5_2: bool = ((s_5_0) == (s_5_1));
        // D s_5_3: not s_5_2
        let s_5_3: bool = !s_5_2;
        // N s_5_4: branch s_5_3 b9 b6
        if s_5_3 {
            return block_9(state, tracer, fn_state);
        } else {
            return block_6(state, tracer, fn_state);
        };
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_6_0: const #12816u : u32
        let s_6_0: u32 = 12816;
        // D s_6_1: read-reg s_6_0:struct
        let s_6_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_6_0 as isize);
            tracer.read_register(s_6_0 as isize, value);
            value
        };
        // D s_6_2: call _get_TCR_EL2_Type_A1(s_6_1)
        let s_6_2: bool = u_get_TCR_EL2_Type_A1(state, tracer, s_6_1);
        // D s_6_3: cast zx s_6_2 -> bv
        let s_6_3: Bits = Bits::new(s_6_2 as u128, 1u16);
        // C s_6_4: const #0u : u8
        let s_6_4: bool = false;
        // C s_6_5: cast zx s_6_4 -> bv
        let s_6_5: Bits = Bits::new(s_6_4 as u128, 1u16);
        // D s_6_6: cmp-eq s_6_3 s_6_5
        let s_6_6: bool = ((s_6_3) == (s_6_5));
        // N s_6_7: branch s_6_6 b8 b7
        if s_6_6 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_7(state, tracer, fn_state);
        };
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_7_0: const #22904u : u32
        let s_7_0: u32 = 22904;
        // D s_7_1: read-reg s_7_0:struct
        let s_7_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_7_0 as isize);
            tracer.read_register(s_7_0 as isize, value);
            value
        };
        // D s_7_2: call _get_MECID_P0_EL2_Type_MECID(s_7_1)
        let s_7_2: u16 = u_get_MECID_P0_EL2_Type_MECID(state, tracer, s_7_1);
        // D s_7_3: write-var return_value <= s_7_2
        fn_state.return_value = s_7_2;
        // N s_7_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_8_0: const #90168u : u32
        let s_8_0: u32 = 90168;
        // D s_8_1: read-reg s_8_0:struct
        let s_8_1: ProductType5c790c8ef59cc8b2 = {
            let value = state
                .read_register::<ProductType5c790c8ef59cc8b2>(s_8_0 as isize);
            tracer.read_register(s_8_0 as isize, value);
            value
        };
        // D s_8_2: call _get_MECID_P1_EL2_Type_MECID(s_8_1)
        let s_8_2: u16 = u_get_MECID_P1_EL2_Type_MECID(state, tracer, s_8_1);
        // D s_8_3: write-var return_value <= s_8_2
        fn_state.return_value = s_8_2;
        // N s_8_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_9_0: const #4u : u32
        let s_9_0: u32 = 4;
        // D s_9_1: read-var regime:u32
        let s_9_1: u32 = fn_state.regime;
        // D s_9_2: cmp-eq s_9_0 s_9_1
        let s_9_2: bool = ((s_9_0) == (s_9_1));
        // D s_9_3: not s_9_2
        let s_9_3: bool = !s_9_2;
        // N s_9_4: branch s_9_3 b11 b10
        if s_9_3 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_10(state, tracer, fn_state);
        };
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
        // N s_10_4: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_11_0: const #() : ()
        let s_11_0: () = ();
        // S s_11_1: call Unreachable(s_11_0)
        let s_11_1: () = Unreachable(state, tracer, s_11_0);
        // D s_11_2: read-var ga#14531:u16
        let s_11_2: u16 = fn_state.ga_14531;
        // D s_11_3: write-var return_value <= s_11_2
        fn_state.return_value = s_11_2;
        // N s_11_4: jump b4
        return block_4(state, tracer, fn_state);
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
        // N s_12_3: jump b4
        return block_4(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> u16 {
        // C s_13_0: const #832u : u32
        let s_13_0: u32 = 832;
        // D s_13_1: read-reg s_13_0:u16
        let s_13_1: u16 = {
            let value = state.read_register::<u16>(s_13_0 as isize);
            tracer.read_register(s_13_0 as isize, value);
            value
        };
        // D s_13_2: write-var return_value <= s_13_1
        fn_state.return_value = s_13_1;
        // N s_13_3: jump b4
        return block_4(state, tracer, fn_state);
    }
}
