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
use u__WriteGIC::*;
use CreatePhysMemRetStatus::*;
use sail_mem_write::*;
use ZeroExtend1::*;
use write_request::*;
use u__WriteUART::*;
use common::*;
pub fn PhysMemWrite<T: Tracer>(
    state: &mut State,
    tracer: &T,
    desc: ProductTypece7c66ccb2cab13e,
    size: i128,
    accdesc: ProductType9878976b5bcce9c9,
    translation_info: SumTypeb20592b6489a79bd,
    data: Bits,
) -> ProductTypef8c3639b88223255 {
    #[derive(Default)]
    struct FunctionState {
        gs_331564: bool,
        return_value: ProductTypef8c3639b88223255,
        ga_371031: SumType7151e9c01acfacea,
        ga_371027: ProductTypeda0231e9dc169f81,
        gs_331571: ProductTypef8c3639b88223255,
        ga_371032: ProductTypeda0231e9dc169f81,
        paddress: u64,
        desc: ProductTypece7c66ccb2cab13e,
        size: i128,
        accdesc: ProductType9878976b5bcce9c9,
        translation_info: SumTypeb20592b6489a79bd,
        data: Bits,
    }
    let fn_state = FunctionState {
        desc,
        size,
        accdesc,
        translation_info,
        data,
        ..Default::default()
    };
    return block_0(state, tracer, fn_state);
    fn block_0<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef8c3639b88223255 {
        // D s_0_0: read-var desc.3:struct
        let s_0_0: ProductTypeda0231e9dc169f81 = fn_state.desc._3;
        // D s_0_1: write-var ga#371032 <= s_0_0
        fn_state.ga_371032 = s_0_0;
        // D s_0_2: read-var ga#371032.0:struct
        let s_0_2: u64 = fn_state.ga_371032._0;
        // D s_0_3: write-var paddress <= s_0_2
        fn_state.paddress = s_0_2;
        // C s_0_4: const #4s : i
        let s_0_4: i128 = 4;
        // D s_0_5: read-var size:i
        let s_0_5: i128 = fn_state.size;
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
    ) -> ProductTypef8c3639b88223255 {
        // C s_1_0: const #0u : u8
        let s_1_0: bool = false;
        // D s_1_1: write-var gs#331564 <= s_1_0
        fn_state.gs_331564 = s_1_0;
        // N s_1_2: jump b2
        return block_2(state, tracer, fn_state);
    }
    fn block_2<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef8c3639b88223255 {
        // D s_2_0: read-var gs#331564:u8
        let s_2_0: bool = fn_state.gs_331564;
        // N s_2_1: branch s_2_0 b12 b3
        if s_2_0 {
            return block_12(state, tracer, fn_state);
        } else {
            return block_3(state, tracer, fn_state);
        };
    }
    fn block_3<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef8c3639b88223255 {
        // C s_3_0: const #16s : i
        let s_3_0: i128 = 16;
        // C s_3_1: const #40s : i
        let s_3_1: i128 = 40;
        // D s_3_2: read-var paddress:u56
        let s_3_2: u64 = fn_state.paddress;
        // D s_3_3: cast zx s_3_2 -> bv
        let s_3_3: Bits = Bits::new(s_3_2 as u128, 56u16);
        // D s_3_4: bit-extract s_3_3 s_3_0 s_3_1
        let s_3_4: Bits = (Bits::new(
            ((s_3_3) >> (s_3_0)).value(),
            u16::try_from(s_3_1).unwrap(),
        ));
        // D s_3_5: cast reint s_3_4 -> u40
        let s_3_5: u64 = (s_3_4.value() as u64);
        // C s_3_6: const #39s : i64
        let s_3_6: i64 = 39;
        // C s_3_7: const #1s : i
        let s_3_7: i128 = 1;
        // C s_3_8: cast zx s_3_6 -> i
        let s_3_8: i128 = (i128::try_from(s_3_6).unwrap());
        // C s_3_9: add s_3_8 s_3_7
        let s_3_9: i128 = (s_3_8 + s_3_7);
        // C s_3_10: cast reint s_3_9 -> i64
        let s_3_10: i64 = (s_3_9 as i64);
        // C s_3_11: cast zx s_3_10 -> i
        let s_3_11: i128 = (i128::try_from(s_3_10).unwrap());
        // C s_3_12: const #1352u : u32
        let s_3_12: u32 = 1352;
        // D s_3_13: read-reg s_3_12:u16
        let s_3_13: u16 = {
            let value = state.read_register::<u16>(s_3_12 as isize);
            tracer.read_register(s_3_12 as isize, value);
            value
        };
        // D s_3_14: cast zx s_3_13 -> bv
        let s_3_14: Bits = Bits::new(s_3_13 as u128, 16u16);
        // D s_3_15: call ZeroExtend1(s_3_11, s_3_14)
        let s_3_15: Bits = ZeroExtend1(state, tracer, s_3_11, s_3_14);
        // D s_3_16: cast reint s_3_15 -> u40
        let s_3_16: u64 = (s_3_15.value() as u64);
        // D s_3_17: cast zx s_3_5 -> bv
        let s_3_17: Bits = Bits::new(s_3_5 as u128, 40u16);
        // D s_3_18: cast zx s_3_16 -> bv
        let s_3_18: Bits = Bits::new(s_3_16 as u128, 40u16);
        // D s_3_19: cmp-eq s_3_17 s_3_18
        let s_3_19: bool = ((s_3_17) == (s_3_18));
        // N s_3_20: branch s_3_19 b11 b4
        if s_3_19 {
            return block_11(state, tracer, fn_state);
        } else {
            return block_4(state, tracer, fn_state);
        };
    }
    fn block_4<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef8c3639b88223255 {
        // D s_4_0: read-var desc.7:struct
        let s_4_0: u64 = fn_state.desc._7;
        // D s_4_1: read-var desc.3:struct
        let s_4_1: ProductTypeda0231e9dc169f81 = fn_state.desc._3;
        // D s_4_2: write-var ga#371027 <= s_4_1
        fn_state.ga_371027 = s_4_1;
        // D s_4_3: read-var ga#371027.0:struct
        let s_4_3: u64 = fn_state.ga_371027._0;
        // D s_4_4: read-var accdesc:struct
        let s_4_4: ProductType9878976b5bcce9c9 = fn_state.accdesc;
        // D s_4_5: read-var translation_info:enum
        let s_4_5: SumTypeb20592b6489a79bd = fn_state.translation_info;
        // D s_4_6: read-var size:i
        let s_4_6: i128 = fn_state.size;
        // D s_4_7: read-var data:bv
        let s_4_7: Bits = fn_state.data;
        // D s_4_8: call write_request(s_4_4, s_4_5, s_4_6, s_4_0, s_4_3, s_4_7)
        let s_4_8: ProductType5c38c56b0a400358 = write_request(
            state,
            tracer,
            s_4_4,
            s_4_5,
            s_4_6,
            s_4_0,
            s_4_3,
            s_4_7,
        );
        // D s_4_9: call sail_mem_write(s_4_8)
        let s_4_9: SumType7151e9c01acfacea = sail_mem_write(state, tracer, s_4_8);
        // D s_4_10: write-var ga#371031 <= s_4_9
        fn_state.ga_371031 = s_4_9;
        // D s_4_11: read-var ga#371031:enum
        let s_4_11: SumType7151e9c01acfacea = fn_state.ga_371031;
        // D s_4_12: matches-sum s_4_11 1
        let s_4_12: bool = matches!(s_4_11, SumType7151e9c01acfacea::_1(_));
        // N s_4_13: branch s_4_12 b8 b5
        if s_4_12 {
            return block_8(state, tracer, fn_state);
        } else {
            return block_5(state, tracer, fn_state);
        };
    }
    fn block_5<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef8c3639b88223255 {
        // C s_5_0: const #0u : u32
        let s_5_0: u32 = 0;
        // S s_5_1: call CreatePhysMemRetStatus(s_5_0)
        let s_5_1: ProductTypef8c3639b88223255 = CreatePhysMemRetStatus(
            state,
            tracer,
            s_5_0,
        );
        // D s_5_2: write-var gs#331571 <= s_5_1
        fn_state.gs_331571 = s_5_1;
        // N s_5_3: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_6<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef8c3639b88223255 {
        // D s_6_0: read-var gs#331571:struct
        let s_6_0: ProductTypef8c3639b88223255 = fn_state.gs_331571;
        // D s_6_1: write-var return_value <= s_6_0
        fn_state.return_value = s_6_0;
        // N s_6_2: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_7<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef8c3639b88223255 {
        // D s_7_0: read-var return_value:struct
        let s_7_0: ProductTypef8c3639b88223255 = fn_state.return_value;
        // N s_7_1: return s_7_0
        return s_7_0;
    }
    fn block_8<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef8c3639b88223255 {
        // D s_8_0: read-var ga#371031:enum
        let s_8_0: SumType7151e9c01acfacea = fn_state.ga_371031;
        // D s_8_1: matches-sum s_8_0 0
        let s_8_1: bool = matches!(s_8_0, SumType7151e9c01acfacea::_0(_));
        // N s_8_2: branch s_8_1 b10 b9
        if s_8_1 {
            return block_10(state, tracer, fn_state);
        } else {
            return block_9(state, tracer, fn_state);
        };
    }
    fn block_9<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef8c3639b88223255 {
        // D s_9_0: read-var ga#371031:enum
        let s_9_0: SumType7151e9c01acfacea = fn_state.ga_371031;
        // D s_9_1: unwrap-sum s_9_0 0
        let s_9_1: u32 = match s_9_0 {
            SumType7151e9c01acfacea::_0(inner) => inner,
            _ => panic!("unwrap sum failed"),
        };
        // D s_9_2: call CreatePhysMemRetStatus(s_9_1)
        let s_9_2: ProductTypef8c3639b88223255 = CreatePhysMemRetStatus(
            state,
            tracer,
            s_9_1,
        );
        // D s_9_3: write-var gs#331571 <= s_9_2
        fn_state.gs_331571 = s_9_2;
        // N s_9_4: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_10<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef8c3639b88223255 {
        // N s_10_0: jump b6
        return block_6(state, tracer, fn_state);
    }
    fn block_11<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef8c3639b88223255 {
        // C s_11_0: const #0s : i
        let s_11_0: i128 = 0;
        // D s_11_1: read-var paddress:u56
        let s_11_1: u64 = fn_state.paddress;
        // D s_11_2: cast zx s_11_1 -> bv
        let s_11_2: Bits = Bits::new(s_11_1 as u128, 56u16);
        // C s_11_3: const #1s : i64
        let s_11_3: i64 = 1;
        // C s_11_4: cast zx s_11_3 -> i
        let s_11_4: i128 = (i128::try_from(s_11_3).unwrap());
        // C s_11_5: const #15s : i
        let s_11_5: i128 = 15;
        // C s_11_6: add s_11_5 s_11_4
        let s_11_6: i128 = (s_11_5 + s_11_4);
        // D s_11_7: bit-extract s_11_2 s_11_0 s_11_6
        let s_11_7: Bits = (Bits::new(
            ((s_11_2) >> (s_11_0)).value(),
            u16::try_from(s_11_6).unwrap(),
        ));
        // D s_11_8: cast reint s_11_7 -> u16
        let s_11_8: u16 = (s_11_7.value() as u16);
        // D s_11_9: read-var size:i
        let s_11_9: i128 = fn_state.size;
        // D s_11_10: read-var data:bv
        let s_11_10: Bits = fn_state.data;
        // D s_11_11: call __WriteUART(s_11_8, s_11_9, s_11_10)
        let s_11_11: () = u__WriteUART(state, tracer, s_11_8, s_11_9, s_11_10);
        // C s_11_12: const #0u : u32
        let s_11_12: u32 = 0;
        // S s_11_13: call CreatePhysMemRetStatus(s_11_12)
        let s_11_13: ProductTypef8c3639b88223255 = CreatePhysMemRetStatus(
            state,
            tracer,
            s_11_12,
        );
        // D s_11_14: write-var return_value <= s_11_13
        fn_state.return_value = s_11_13;
        // N s_11_15: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_12<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef8c3639b88223255 {
        // C s_12_0: const #0s : i
        let s_12_0: i128 = 0;
        // D s_12_1: read-var paddress:u56
        let s_12_1: u64 = fn_state.paddress;
        // D s_12_2: cast zx s_12_1 -> bv
        let s_12_2: Bits = Bits::new(s_12_1 as u128, 56u16);
        // C s_12_3: const #1s : i64
        let s_12_3: i64 = 1;
        // C s_12_4: cast zx s_12_3 -> i
        let s_12_4: i128 = (i128::try_from(s_12_3).unwrap());
        // C s_12_5: const #15s : i
        let s_12_5: i128 = 15;
        // C s_12_6: add s_12_5 s_12_4
        let s_12_6: i128 = (s_12_5 + s_12_4);
        // D s_12_7: bit-extract s_12_2 s_12_0 s_12_6
        let s_12_7: Bits = (Bits::new(
            ((s_12_2) >> (s_12_0)).value(),
            u16::try_from(s_12_6).unwrap(),
        ));
        // D s_12_8: cast reint s_12_7 -> u16
        let s_12_8: u16 = (s_12_7.value() as u16);
        // D s_12_9: read-var data:bv
        let s_12_9: Bits = fn_state.data;
        // D s_12_10: cast reint s_12_9 -> u32
        let s_12_10: u32 = (s_12_9.value() as u32);
        // D s_12_11: call __WriteGIC(s_12_8, s_12_10)
        let s_12_11: () = u__WriteGIC(state, tracer, s_12_8, s_12_10);
        // C s_12_12: const #0u : u32
        let s_12_12: u32 = 0;
        // S s_12_13: call CreatePhysMemRetStatus(s_12_12)
        let s_12_13: ProductTypef8c3639b88223255 = CreatePhysMemRetStatus(
            state,
            tracer,
            s_12_12,
        );
        // D s_12_14: write-var return_value <= s_12_13
        fn_state.return_value = s_12_13;
        // N s_12_15: jump b7
        return block_7(state, tracer, fn_state);
    }
    fn block_13<T: Tracer>(
        state: &mut State,
        tracer: &T,
        mut fn_state: FunctionState,
    ) -> ProductTypef8c3639b88223255 {
        // C s_13_0: const #16s : i
        let s_13_0: i128 = 16;
        // C s_13_1: const #40s : i
        let s_13_1: i128 = 40;
        // D s_13_2: read-var paddress:u56
        let s_13_2: u64 = fn_state.paddress;
        // D s_13_3: cast zx s_13_2 -> bv
        let s_13_3: Bits = Bits::new(s_13_2 as u128, 56u16);
        // D s_13_4: bit-extract s_13_3 s_13_0 s_13_1
        let s_13_4: Bits = (Bits::new(
            ((s_13_3) >> (s_13_0)).value(),
            u16::try_from(s_13_1).unwrap(),
        ));
        // D s_13_5: cast reint s_13_4 -> u40
        let s_13_5: u64 = (s_13_4.value() as u64);
        // C s_13_6: const #39s : i64
        let s_13_6: i64 = 39;
        // C s_13_7: const #1s : i
        let s_13_7: i128 = 1;
        // C s_13_8: cast zx s_13_6 -> i
        let s_13_8: i128 = (i128::try_from(s_13_6).unwrap());
        // C s_13_9: add s_13_8 s_13_7
        let s_13_9: i128 = (s_13_8 + s_13_7);
        // C s_13_10: cast reint s_13_9 -> i64
        let s_13_10: i64 = (s_13_9 as i64);
        // C s_13_11: cast zx s_13_10 -> i
        let s_13_11: i128 = (i128::try_from(s_13_10).unwrap());
        // C s_13_12: const #1360u : u32
        let s_13_12: u32 = 1360;
        // D s_13_13: read-reg s_13_12:u16
        let s_13_13: u16 = {
            let value = state.read_register::<u16>(s_13_12 as isize);
            tracer.read_register(s_13_12 as isize, value);
            value
        };
        // D s_13_14: cast zx s_13_13 -> bv
        let s_13_14: Bits = Bits::new(s_13_13 as u128, 16u16);
        // D s_13_15: call ZeroExtend1(s_13_11, s_13_14)
        let s_13_15: Bits = ZeroExtend1(state, tracer, s_13_11, s_13_14);
        // D s_13_16: cast reint s_13_15 -> u40
        let s_13_16: u64 = (s_13_15.value() as u64);
        // D s_13_17: cast zx s_13_5 -> bv
        let s_13_17: Bits = Bits::new(s_13_5 as u128, 40u16);
        // D s_13_18: cast zx s_13_16 -> bv
        let s_13_18: Bits = Bits::new(s_13_16 as u128, 40u16);
        // D s_13_19: cmp-eq s_13_17 s_13_18
        let s_13_19: bool = ((s_13_17) == (s_13_18));
        // D s_13_20: write-var gs#331564 <= s_13_19
        fn_state.gs_331564 = s_13_19;
        // N s_13_21: jump b2
        return block_2(state, tracer, fn_state);
    }
}
